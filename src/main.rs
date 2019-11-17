#![feature(const_generics, const_generic_impls_guard)]
#![allow(incomplete_features)] // I know const generics is still buggy...

use std::{
    array::LengthAtMost32,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use pbr::ProgressBar;
use structopt::StructOpt;
use term_painter::{ToStyle, Color::*};

use crate::{
    analyze::Analyzer,
    cli::Args,
    summary::Summary,
    tape::CellValue,
    tm::{AllTmCombinations, State},
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
        _ => {
            eprintln!(
                "Currently, only `-n` up to 4 are allowed. This whole problem \
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
    [State; N]: LengthAtMost32,
    [u16; N]: LengthAtMost32,
    [bool; N]: LengthAtMost32,
{
    // Iterator to iterate over all possible TMs.
    let mut tms = <AllTmCombinations<{N}>>::new();
    let num_tms = tms.len();

    println!("");
    Blue.bold().with(|| println!("▸ Analyzing all {} TMs with {} states...", num_tms, N));
    println!("");


    // ----- Run ---------------------------------------------------
    let summary = Summary::new(args, num_tms as u64);
    let summary = Arc::new(Mutex::new(summary));
    let mut pb = ProgressBar::new(num_tms as u64);
    pb.set_max_refresh_rate(Some(Duration::from_millis(10)));
    let pb = Arc::new(Mutex::new(pb));

    let before = Instant::now();

    // Create a channel to pass pass the work to the workers. We bound it to
    // three to have always have some work ready, but to not use too much
    // memory.
    let (s, r) = crossbeam_channel::bounded::<AllTmCombinations<{N}>>(3);

    // Create the worker threads
    let join_handles = (0..num_cpus::get()).map(|_| {
        let new_jobs = r.clone();
        let summary = summary.clone();
        let pb = pb.clone();
        let args = args.clone();
        thread::spawn(move || {
            // Cache this vector here.
            let mut outcomes = Vec::new();
            let mut analyzer = Analyzer::new(args);

            for job in new_jobs.iter() {
                outcomes.clear();
                let job_len = job.len() as u64;

                // Analyze each TM in this batch
                for tm in job {
                    let outcome = analyzer.analyze(&tm);
                    outcomes.push(outcome);
                }

                // Handle the outcomes
                let mut summary = summary.lock().expect("poisened lock");
                for outcome in &outcomes {
                    summary.handle_outcome(*outcome);
                }

                // Advance progress bar
                pb.lock().expect("poisened lock").add(job_len);
            }
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
        // let job = tms.by_ref().take(chunk_size).collect::<Vec<_>>();
        s.send(job).expect("channel unexpectedly disconnected");
    }

    // Join all threads
    drop(s);
    for handle in join_handles {
        handle.join().expect("panic in worker thread");
    }

    if !args.no_pb {
        pb.lock().unwrap().finish();
        println!();
    }

    println!();
    println!("  (That took {:.2?})", before.elapsed());

    // ----- Print results ---------------------------------------------------
    println!();
    summary.lock().unwrap().print_report();
}


/// The outcome of simulating a TM.
#[derive(Debug, Clone, Copy)]
pub enum Outcome {
    // ----- Outcomes from actually running the TM ---------------------------
    /// The TM ran and halted.
    Halted {
        steps: u64,
        ones: u64,
    },

    /// The TM ran but was aborted after the maximum number of steps.
    AbortedAfterMaxSteps,


    // ----- Outcomes from static analysis -----------------------------------
    /// The start state of the TM for the cell value 0 has the halt state as
    /// next state. This means the TM terminates in one step. It might write a
    /// single one, but we just ignore that information.
    ImmediateHalt,

    /// The TM does not even have a transition to the halt state at all.
    NoHaltState,

    /// If the turing machine has a state graph where the halt state cannot be
    /// reached from the start state.
    HaltStateNotReachable,
}
