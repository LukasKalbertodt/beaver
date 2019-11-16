#![feature(const_generics, const_generic_impls_guard)]
#![allow(incomplete_features)] // I know const generics is still buggy...

use std::{
    array::LengthAtMost32,
    cmp::min,
};

use pbr::ProgressBar;
use structopt::StructOpt;
use term_painter::{ToStyle, Color::*};

use crate::{
    cli::Args,
    tape::{CellId, CellValue, Tape},
    tm::{AllTmCombinations, Move, State, Tm, HALT_STATE},
};


mod cli;
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
fn run<const N: usize>(args: &Args)
where
    [State; N]: LengthAtMost32,
    [usize; N]: LengthAtMost32,
{
    // Iterator to iterate over all possible TMs.
    let tms = <AllTmCombinations<{N}>>::new();
    let num_tms = tms.len();

    println!("");
    Blue.bold().with(|| println!("▸ Simulating all {} TMs with {} states...", num_tms, N));
    println!("");



    // ----- Run an analyze ---------------------------------------------------
    let mut high_score = 0;
    let mut num_winners = 0;
    let mut fewest_winner_steps = 0;
    let mut num_aborted = 0;

    let mut pb = ProgressBar::new(num_tms as u64);
    pb.set_max_refresh_rate(Some(std::time::Duration::from_millis(10)));

    for (i, tm) in tms.enumerate() {
        let outcome = run_tm(&tm, args);
        match outcome {
            Outcome::Halted { steps, ones } => {
                if ones > high_score {
                    high_score = ones;
                    num_winners = 1;
                    fewest_winner_steps = steps;
                } else if ones == high_score {
                    num_winners += 1;
                    fewest_winner_steps = min(fewest_winner_steps, steps);
                }
            }
            Outcome::StoppedAfterMaxSteps => num_aborted += 1,
        }

        let at_once = if N >= 3 { 1000 } else { 1 };
        if i % at_once == 0 {
            pb.add(at_once as u64);
        }
    }


    // ----- Print results ---------------------------------------------------
    println!();
    println!();
    Blue.bold().with(|| println!("▸ Results:"));
    println!(
        "- The high score (number of 1s after halting) is: {}",
        Green.bold().paint(high_score),
    );
    println!("  - {} TMs reached that high score", Green.bold().paint(num_winners));
    println!(
        "  - The quickest of which reached the high score in {} steps",
        Green.bold().paint(fewest_winner_steps),
    );
    println!(
        "- {} TMs halted but did not get a high score",
        Yellow.bold().paint(num_tms - num_aborted),
    );
    println!(
        "- {} were aborted after the maximum number of steps ({})",
        Red.bold().paint(num_aborted),
        args.max_steps,
    );
}


/// The outcome of simulating a TM.
#[derive(Debug, Clone, Copy)]
enum Outcome {
    Halted {
        steps: u64,
        ones: u64,
    },
    StoppedAfterMaxSteps,
}

/// Simulate a turing machine.
fn run_tm<const N: usize>(tm: &Tm<{N}>, args: &Args) -> Outcome {
    let mut tape = Tape::new();
    let mut head = CellId(0);
    let mut current_state: u8 = 0;

    let mut steps = 0;
    loop {
        steps += 1;

        let value = tape.get(head);
        let action = tm.states[current_state as usize].action_for(value);
        tape.write(head, action.write);

        if action.next_state == HALT_STATE {
            break;
        }

        current_state = action.next_state;
        match action.movement {
            Move::Left => head.0 -= 1,
            Move::Right => head.0 += 1,
        }

        if steps == args.max_steps {
            return Outcome::StoppedAfterMaxSteps;
        }
    }


    let r = tape.written_range();
    let ones = (r.start.0..r.end.0).filter(|&id| tape.get(CellId(id)).0).count() as u64;

    Outcome::Halted { steps, ones }
}