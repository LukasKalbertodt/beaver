use anyhow::Result;
use std::{cmp::min, ops::Range, sync::{Arc, Mutex}, thread, time::{Duration, Instant}};
use structopt::StructOpt;
use pbr::ProgressBar;

use crate::{SharedArgs, analyze::Analyzer, gen::{All, TmGenerator}};

mod summary;

use self::summary::Summary;


#[derive(StructOpt, Debug, Clone)]
pub struct Args {
    #[structopt(flatten)]
    shared: SharedArgs,

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


pub(crate) fn run(args: Args) -> Result<()> {
    match args.shared.n {
        1 => do_run::<All<1>, 1>(args),
        2 => do_run::<All<2>, 2>(args),
        3 => do_run::<All<3>, 3>(args),
        4 => do_run::<All<4>, 4>(args),
        5 => do_run::<All<5>, 5>(args),
        6 => do_run::<All<6>, 6>(args),
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
                    let outcome = analyzer.analyze(tm);
                    summary.handle_outcome(outcome);
                });

                // Advance progress bar
                if !args.no_pb {
                    pb.lock().expect("poisened lock").add(job_len);
                }
            }

            summary
        })
    }).collect::<Vec<_>>();

    let chunk_size = match N {
        1 => 1,
        2 => 1000,
        3 => 10_000,
        _ => 100_000,
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

