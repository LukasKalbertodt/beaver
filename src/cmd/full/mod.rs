use anyhow::Result;
use std::{
    cmp::min,
    ops::Range,
    str::FromStr,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};
use structopt::StructOpt;
use pbr::ProgressBar;

use crate::{SharedArgs, analyze::Analyzer, gen::{All, NoSymmetries, Optimized, TmGenerator}};

mod summary;

use self::summary::Summary;


#[derive(StructOpt, Debug, Clone)]
pub struct Args {
    #[structopt(flatten)]
    shared: SharedArgs,

    /// Set TM generator. 'all' blindly generates all possible TMs; 'no-symmetries'
    /// eliminates symmetric TMs that will result in the same outcome; 'optimized'
    /// also eliminates TMs that have on chance of winning busy beaver.
    #[structopt(short, long, default_value = "optimized")]
    generator: Generator,

    /// If specified, the progress bar is not shown.
    #[structopt(long)]
    pub no_pb: bool,

    /// Number of threads to use. Defaults to the number of virtual CPUs.
    #[structopt(long, short = "j")]
    pub num_threads: Option<u32>,

    /// Set the height of the histogram that's printed in the end.
    #[structopt(long, default_value = "15")]
    pub histogram_height: u32,

    /// Set the max number of steps included in the histogram that's printed in
    /// the end.
    #[structopt(long, default_value = "30")]
    pub histogram_cutoff: u32,

    /// If specified, the histogram is now shown.
    #[structopt(long)]
    pub hide_histogram: bool,
}

#[derive(Debug, Clone)]
enum Generator {
    All,
    NoSymmetries,
    Optimized,
}

impl FromStr for Generator {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" => Ok(Self::All),
            "no-symmetries" => Ok(Self::NoSymmetries),
            "optimized" => Ok(Self::Optimized),
            _ => Err("invalid value for 'generator'"),
        }
    }
}


pub(crate) fn run(args: Args) -> Result<()> {
    macro_rules! dispatch_generator {
        ($n:expr) => {
            match args.generator {
                Generator::All => do_run::<All<$n>, $n>(args),
                Generator::NoSymmetries => do_run::<NoSymmetries<$n>, $n>(args),
                Generator::Optimized => do_run::<Optimized<$n>, $n>(args),
            }
        };
    }

    match args.shared.n {
        1 => dispatch_generator!(1),
        2 => dispatch_generator!(2),
        3 => dispatch_generator!(3),
        4 => dispatch_generator!(4),
        5 => dispatch_generator!(5),
        6 => dispatch_generator!(6),
        _ => panic!("invalid value for n: argument parsing should catch this"),
    }

    Ok(())
}

#[inline(never)] // Useful for inspecting assembly
fn do_run<G: TmGenerator<N>, const N: usize>(args: Args)
where
    [bool; N]: Default,
{
    let num_tms = G::num_tms();
    println!("");
    bunt::println!(
        "{$blue+bold}▸ Analyzing {[intense]} TMs with {[intense]} states...{/$}",
        num_tms,
        N,
    );
    println!("");
    println!("... using the generator '{}'", G::description());
    println!("");


    // ----- Run -------------------------------------------------------------
    let mut pb = ProgressBar::new(num_tms);
    pb.set_max_refresh_rate(Some(Duration::from_millis(10)));
    let pb = Arc::new(Mutex::new(pb));

    let before = Instant::now();

    // Create a channel to pass pass the work to the workers. We bound it to 32
    // to have always have some work ready, but to not use too much memory.
    let (s, r) = crossbeam_channel::bounded::<Range<u64>>(32);

    // Create the worker threads
    let num_threads = args.num_threads.unwrap_or_else(|| num_cpus::get() as u32);
    let join_handles = (0..num_threads).map(|_| {
        let new_jobs = r.clone();
        let pb = pb.clone();
        let args = args.clone();
        let args = args.clone();
        thread::spawn(move || {
            let mut analyzer = Analyzer::new(args.shared.max_steps);
            let mut summary = Summary::new();

            for range in new_jobs.iter() {
                let job_len = range.end - range.start;

                // Analyze each TM in this batch
                let mut count = 0;
                G::for_range(range.clone(), |tm| {
                    count += 1;
                    analyzer.analyze(tm, &mut summary);
                });

                // Advance progress bar
                if !args.no_pb {
                    pb.lock().expect("poisened lock").add(job_len);
                }
            }

            summary
        })
    }).collect::<Vec<_>>();

    // So in theory, a very large number is best for performance. BUT the
    // progress bar only changes when a whole chunk is done. So for super slow
    // PCs, or debug builds, or runs with lots of debug output, or stuff like
    // that -- we want the progress bar to still be useful.
    let chunk_size = match N {
        1 => 1,
        2 => 500,
        3 => 50_000,
        _ => 1_000_000,
    };

    for start in (0..num_tms).step_by(chunk_size) {
        let range = start..min(start + chunk_size as u64, num_tms);
        s.send(range).expect("channel unexpectedly disconnected");
    }

    // Join all threads
    drop(s);
    let mut summary = Summary::new();
    for handle in join_handles {
        let thread_summary = handle.join().expect("panic in worker thread");
        summary.add(thread_summary);
    }

    if !args.no_pb {
        pb.lock().unwrap().finish();
        println!();
    }

    println!();
    let elapsed = before.elapsed();

    // The `as u64` could technically overflow, but 2^64ns = 584 years, so...
    let core_nanos_per_tm = (elapsed.as_nanos() * num_threads as u128) / num_tms as u128;
    let core_time_per_tm = Duration::from_nanos(core_nanos_per_tm as u64);
    println!(
        "  (That took {:.2?}, {:?} per TM on {} threads -> {:?} core time per TM)",
        elapsed,
        core_time_per_tm / num_threads,
        num_threads,
        core_time_per_tm,
    );

    // ----- Print results ---------------------------------------------------
    println!();
    summary.print_report(&args);
}

