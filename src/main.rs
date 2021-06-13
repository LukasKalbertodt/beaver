use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use pbr::ProgressBar;
use structopt::StructOpt;

use crate::{
    analyze::Analyzer,
    cli::Args,
    summary::Summary,
    tape::CellValue,
    tm::AllTmCombinations,
};

mod analyze;
mod cli;
mod summary;
mod tape;
mod tm;



fn main() {
    let args = Args::from_args();

    match args.n {
        0 => {
            eprintln!("`-n` must be above 0! Zero state TMs do not make sense.");
            return;
        }
        1 => run::<1>(&args),
        2 => run::<2>(&args),
        3 => run::<3>(&args),
        4 => run::<4>(&args),
        5 => run::<5>(&args),
        _ => {
            eprintln!(
                "Currently, only `-n` up to 5 are allowed. This whole problem \
                    is about quickly growing functions, you know..."
            );
            return;
        }
    }
}

/// Runs the experiment for a given `N`.
#[inline(never)]
fn run<const N: usize>(args: &Args)
where
    [bool; N]: Default,
{
    // Iterator to iterate over all possible TMs.
    let mut tms = <AllTmCombinations<N>>::new();
    let num_tms = tms.len();

    println!("");
    bunt::println!("{$blue+bold}▸ Analyzing all {} TMs with {} states...{/$}", num_tms, N);
    println!("");


    // ----- Run ---------------------------------------------------
    let mut pb = ProgressBar::new(num_tms as u64);
    pb.set_max_refresh_rate(Some(Duration::from_millis(10)));
    let pb = Arc::new(Mutex::new(pb));

    let before = Instant::now();

    // Create a channel to pass pass the work to the workers. We bound it to
    // three to have always have some work ready, but to not use too much
    // memory.
    let (s, r) = crossbeam_channel::bounded::<AllTmCombinations<N>>(3);

    // Create the worker threads
    let num_threads = args.num_threads.unwrap_or_else(|| num_cpus::get() as u32);
    let join_handles = (0..num_threads).map(|_| {
        let new_jobs = r.clone();
        let pb = pb.clone();
        let args = args.clone();
        thread::spawn(move || {
            let mut analyzer = Analyzer::new(args.clone());
            let mut summary = Summary::new();

            for job in new_jobs.iter() {
                let job_len = job.len() as u64;

                // Analyze each TM in this batch
                for tm in job {
                    let outcome = analyzer.analyze(&tm);
                    if args.print_aborted && outcome.was_aborted() {
                        println!("{:?} => {:#?}", outcome, tm);
                    }
                    summary.handle_outcome(outcome);
                }

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
        2 => 100,
        3 => 10_000,
        _ => 100_000,
    };
    while tms.len() > 0 {
        let job = tms.split_off(chunk_size);
        s.send(job).expect("channel unexpectedly disconnected");
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
    summary.print_report(args);
}


/// The outcome of simulating a TM.
#[derive(Debug, Clone, Copy)]
pub enum Outcome {
    // ----- Outcomes from actually running the TM ---------------------------
    /// The TM ran and halted.
    Halted {
        steps: u32,
        ones: u32,
    },

    /// The TM ran but was aborted after the maximum number of steps.
    AbortedAfterMaxSteps,


    // ----- Outcomes from static analysis -----------------------------------
    /// The start state of the TM for the cell value 0 has the halt state as
    /// next state. This means the TM terminates in one step. It might write a
    /// single one, though.
    ImmediateHalt {
        wrote_one: bool,
    },

    /// The TM does not even have a transition to the halt state at all.
    NoHaltState,

    /// The TM does immediately go into one direction without ever stopping.
    /// This happens if the start action has `next_state == 0`.
    SimpleElope,

    /// If the turing machine has a state graph where the halt state cannot be
    /// reached from the start state.
    HaltStateNotReachable,

    /// While executing the TM a run-away was detected, meaning that the TM
    /// was caught in a loop only visiting new cells, thus never terminating.
    RunAwayDetected,
}

impl Outcome {
    fn was_aborted(&self) -> bool {
        if let Outcome::AbortedAfterMaxSteps = *self {
            true
        } else {
            false
        }
    }
}
