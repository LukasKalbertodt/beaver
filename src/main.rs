#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#![feature(array_map)]

#![allow(incomplete_features)]

// use std::{
//     sync::{Arc, Mutex},
//     thread,
//     time::{Duration, Instant},
// };

// use pbr::ProgressBar;
use structopt::StructOpt;

use crate::{
//     analyze::Analyzer,
    cli::Args,
//     summary::Summary,
//     tape::CellValue,
//     tm::AllTmCombinations,
};

// mod analyze;
mod cli;
// mod summary;
// mod tape;
// mod tm;



fn main() {
    let args = Args::from_args();

    let print = |tm| {
        println!("{:016x?}", tm);
    };

    match args.n {
        0 => {
            eprintln!("`-n` must be above 0! Zero state TMs do not make sense.");
            return;
        }
        1 => for_all_tms::<_, 1>(print),
        2 => for_all_tms::<_, 2>(print),
        3 => for_all_tms::<_, 3>(print),
        4 => for_all_tms::<_, 4>(print),
        5 => for_all_tms::<_, 5>(print),
        _ => {
            eprintln!(
                "Currently, only `-n` up to 5 are allowed. This whole problem \
                    is about quickly growing functions, you know..."
            );
            return;
        }
    }

    // match args.n {
    //     0 => {
    //         eprintln!("`-n` must be above 0! Zero state TMs do not make sense.");
    //         return;
    //     }
    //     1 => run::<1>(&args),
    //     2 => run::<2>(&args),
    //     3 => run::<3>(&args),
    //     4 => run::<4>(&args),
    //     5 => run::<5>(&args),
    //     _ => {
    //         eprintln!(
    //             "Currently, only `-n` up to 5 are allowed. This whole problem \
    //                 is about quickly growing functions, you know..."
    //         );
    //         return;
    //     }
    // }
}

#[inline(never)]
fn for_all_tms<F: FnMut(core::arch::x86_64::__m128i), const N: usize>(mut f: F)
where
    [(); 2 * N]:,
{
    use std::arch::x86_64::*;

    struct Consts {
        one: __m128i,
        halt_add: __m128i,
        clear_mask: __m128i,
    }

    // Create constant m128i values that are different for each "digit".
    let consts = {
        let mut indices = [0usize; 2 * N];
        for (i, e) in indices.iter_mut().enumerate() {
            *e = i;
        }

        macro_rules! shifted_vector {
            ($shift:expr, $yes:expr, $no:expr) => {
                _mm_set_epi8(
                    if $shift == 15 { $yes } else { $no },
                    if $shift == 14 { $yes } else { $no },
                    if $shift == 13 { $yes } else { $no },
                    if $shift == 12 { $yes } else { $no },
                    if $shift == 11 { $yes } else { $no },
                    if $shift == 10 { $yes } else { $no },
                    if $shift == 9 { $yes } else { $no },
                    if $shift == 8 { $yes } else { $no },
                    if $shift == 7 { $yes } else { $no },
                    if $shift == 6 { $yes } else { $no },
                    if $shift == 5 { $yes } else { $no },
                    if $shift == 4 { $yes } else { $no },
                    if $shift == 3 { $yes } else { $no },
                    if $shift == 2 { $yes } else { $no },
                    if $shift == 1 { $yes } else { $no },
                    if $shift == 0 { $yes } else { $no },
                )
            };
        }

        indices.map(|i| unsafe {
            Consts {
                one: shifted_vector!(i, 1, 0),
                // This is the difference between the first encoded action that
                // moves to the halting state (0xFC) and the last non-halt action.
                halt_add: shifted_vector!(i, (0xFC - (N * 4) + 1) as i8, 0),
                clear_mask: shifted_vector!(i, 0, 0xFFu8 as i8),
            }
        })
    };

    // This is an 2N digit number with base `4 * N + 2` (which is the number of
    // different possible states). `u8` is absolutely sufficient for all N that
    // one would realistically try.
    let base = 4 * N as u8 + 2;
    let mut counter = [0u8; 2 * N];
    unsafe {
        let mut tm = _mm_setzero_si128();
        'outer: for _ in 0..100 {

            // macro_rules! inner {
            //     () => {
            //         for _ in 0..2 * 2 * N {
            //             f(tm);
            //             tm = _mm_add_epi8(tm, consts[0].one);
            //         }

            //         tm = _mm_or_si128(tm, consts[0].halt_mask);
            //         f(tm);
            //         tm = _mm_add_epi8(tm, consts[0].one);
            //         f(tm);
            //         tm = _mm_and_si128(tm, consts[0].clear_mask);
            //     }
            // }

            // for _ in 0..2 * 2 * N {
            //     inner!();
            //     tm = _mm_add_epi8(tm, consts[1].one);
            // }

            // tm = _mm_or_si128(tm, consts[1].halt_mask);
            // inner!();
            // tm = _mm_add_epi8(tm, consts[1].one);
            // inner!();
            // do stuff
            f(tm);

            // Increment counter
            for digit in 0..N {
                // println!("digit {}, counter[digit] = {}", digit, counter[digit]);
                counter[digit] += 1;
                if counter[digit] < 4 * N as u8|| counter[digit] == 4 * N as u8 + 1 {
                    tm = _mm_add_epi8(tm, consts[digit].one);
                } else if counter[digit] == 4 * N as u8 {
                    // println!("   halt mask");
                    tm = _mm_add_epi8(tm, consts[digit].halt_add);
                } else if counter[digit] == base {
                    // println!("   reset");
                    counter[digit] = 0;
                    tm = _mm_and_si128(tm, consts[digit].clear_mask);

                    // If we just incremented the most significant digit, we
                    // completely stop.
                    if digit + 1 == N {
                        break 'outer;
                    } else {
                        continue; // to next more significant digit
                    }
                }

                // If we have not reached the `else` above, there is no overflow
                // into the next digit and we stop.
                break;
            }
        }
    }

    // // This is an N digit number with base `(4 * N + 2)^2` (which is the number
    // // of different possible states).
    // let base = (4 * N + 2).pow(2);
    // let mut counter = [0u16; N];
    // unsafe {
    //     'outer: loop {
    //         let mut tm = _mm_setzero_si128();

    //         macro_rules! inner {
    //             () => {
    //                 for _ in 0..2 * 2 * N {
    //                     f(tm);
    //                     tm = _mm_add_epi8(tm, consts[0].one);
    //                 }

    //                 tm = _mm_or_si128(tm, consts[0].halt_mask);
    //                 f(tm);
    //                 tm = _mm_add_epi8(tm, consts[0].one);
    //                 f(tm);
    //                 tm = _mm_and_si128(tm, consts[0].clear_mask);
    //             }
    //         }

    //         for _ in 0..2 * 2 * N {
    //             inner!();
    //             tm = _mm_add_epi8(tm, consts[1].one);
    //         }

    //         tm = _mm_or_si128(tm, consts[1].halt_mask);
    //         inner!();
    //         tm = _mm_add_epi8(tm, consts[1].one);
    //         inner!();
    //         // do stuff

    //         // Increment counter
    //         for digit in 0..N {
    //             counter[digit] += 1;

    //             // If the digit overflows
    //             if counter[digit] == 10 {
    //                 counter[digit] = 0;

    //                 // If we just incremented the most significant digit, we
    //                 // completely stop.
    //                 if digit + 1 == N {
    //                     break 'outer;
    //                 }
    //             } else {
    //                 break;
    //             }
    //         }
    //     }
    // }
}

// /// Runs the experiment for a given `N`.
// #[inline(never)]
// fn run<const N: usize>(args: &Args)
// where
//     [bool; N]: Default,
// {
//     for tm in <AllTmCombinations<N>>::new().take(50) {
//         print!("{:02x} ", tm.states[0].on_0.encoded);
//         print!("{:02x} ", tm.states[0].on_1.encoded);
//         print!("{:02x} ", tm.states[1].on_0.encoded);
//         print!("{:02x} ", tm.states[1].on_1.encoded);
//         print!("{:02x} ", tm.states[2].on_0.encoded);
//         print!("{:02x} ", tm.states[2].on_1.encoded);
//         println!("  ... {:?}", tm);
//     }
//     panic!();

//     // Iterator to iterate over all possible TMs.
//     let mut tms = <AllTmCombinations<N>>::new();
//     let num_tms = tms.len();

//     println!("");
//     bunt::println!("{$blue+bold}▸ Analyzing all {} TMs with {} states...{/$}", num_tms, N);
//     println!("");


//     // ----- Run ---------------------------------------------------
//     let mut pb = ProgressBar::new(num_tms as u64);
//     pb.set_max_refresh_rate(Some(Duration::from_millis(10)));
//     let pb = Arc::new(Mutex::new(pb));

//     let before = Instant::now();

//     // Create a channel to pass pass the work to the workers. We bound it to
//     // three to have always have some work ready, but to not use too much
//     // memory.
//     let (s, r) = crossbeam_channel::bounded::<AllTmCombinations<N>>(3);

//     // Create the worker threads
//     let num_threads = args.num_threads.unwrap_or_else(|| num_cpus::get() as u32);
//     let join_handles = (0..num_threads).map(|_| {
//         let new_jobs = r.clone();
//         let pb = pb.clone();
//         let args = args.clone();
//         thread::spawn(move || {
//             let mut analyzer = Analyzer::new(args.clone());
//             let mut summary = Summary::new();

//             for job in new_jobs.iter() {
//                 let job_len = job.len() as u64;

//                 // Analyze each TM in this batch
//                 for tm in job {
//                     let outcome = analyzer.analyze(&tm);
//                     if args.print_aborted && outcome.was_aborted() {
//                         println!("{:?} => {:#?}", outcome, tm);
//                     }
//                     summary.handle_outcome(outcome);
//                 }

//                 // Advance progress bar
//                 if !args.no_pb {
//                     pb.lock().expect("poisened lock").add(job_len);
//                 }
//             }

//             summary
//         })
//     }).collect::<Vec<_>>();

//     let chunk_size = match N {
//         1 => 1,
//         2 => 100,
//         3 => 10_000,
//         _ => 100_000,
//     };
//     while tms.len() > 0 {
//         let job = tms.split_off(chunk_size);
//         s.send(job).expect("channel unexpectedly disconnected");
//     }

//     // Join all threads
//     drop(s);
//     let mut summary = Summary::new();
//     for handle in join_handles {
//         let thread_summary = handle.join().expect("panic in worker thread");
//         summary.add(thread_summary);
//     }

//     if !args.no_pb {
//         pb.lock().unwrap().finish();
//         println!();
//     }

//     println!();
//     let elapsed = before.elapsed();

//     // The `as u64` could technically overflow, but 2^64ns = 584 years, so...
//     let core_nanos_per_tm = (elapsed.as_nanos() * num_threads as u128) / num_tms as u128;
//     let core_time_per_tm = Duration::from_nanos(core_nanos_per_tm as u64);
//     println!(
//         "  (That took {:.2?}, {:?} per TM on {} threads -> {:?} core time per TM)",
//         elapsed,
//         core_time_per_tm / num_threads,
//         num_threads,
//         core_time_per_tm,
//     );

//     // ----- Print results ---------------------------------------------------
//     println!();
//     summary.print_report(args);
// }


// /// The outcome of simulating a TM.
// #[derive(Debug, Clone, Copy)]
// pub enum Outcome {
//     // ----- Outcomes from actually running the TM ---------------------------
//     /// The TM ran and halted.
//     Halted {
//         steps: u32,
//         ones: u32,
//     },

//     /// The TM ran but was aborted after the maximum number of steps.
//     AbortedAfterMaxSteps,


//     // ----- Outcomes from static analysis -----------------------------------
//     /// The start state of the TM for the cell value 0 has the halt state as
//     /// next state. This means the TM terminates in one step. It might write a
//     /// single one, though.
//     ImmediateHalt {
//         wrote_one: bool,
//     },

//     /// The TM does not even have a transition to the halt state at all.
//     NoHaltState,

//     /// The TM does immediately go into one direction without ever stopping.
//     /// This happens if the start action has `next_state == 0`.
//     SimpleElope,

//     /// If the turing machine has a state graph where the halt state cannot be
//     /// reached from the start state.
//     HaltStateNotReachable,

//     /// While executing the TM a run-away was detected, meaning that the TM
//     /// was caught in a loop only visiting new cells, thus never terminating.
//     RunAwayDetected,
// }

// impl Outcome {
//     fn was_aborted(&self) -> bool {
//         if let Outcome::AbortedAfterMaxSteps = *self {
//             true
//         } else {
//             false
//         }
//     }
// }
