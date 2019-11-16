#![feature(const_generics, const_generic_impls_guard)]
#![allow(incomplete_features)] // I know const generics is still buggy...

use std::{
    array::LengthAtMost32,
    time::{Duration, Instant},
};

use pbr::ProgressBar;
use structopt::StructOpt;
use term_painter::{ToStyle, Color::*};

use crate::{
    cli::Args,
    summary::Summary,
    tape::{CellId, CellValue, Tape},
    tm::{AllTmCombinations, Move, State, Tm, HALT_STATE},
};


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
fn run<const N: usize>(args: &Args)
where
    [State; N]: LengthAtMost32,
    [usize; N]: LengthAtMost32,
{
    // Iterator to iterate over all possible TMs.
    let tms = <AllTmCombinations<{N}>>::new();
    let num_tms = tms.len();

    println!("");
    Blue.bold().with(|| println!("▸ Analyzing all {} TMs with {} states...", num_tms, N));
    println!("");


    // ----- Run ---------------------------------------------------
    let mut summary = Summary::new(args, num_tms as u64);
    let mut pb = ProgressBar::new(num_tms as u64);
    pb.set_max_refresh_rate(Some(Duration::from_millis(10)));

    let before = Instant::now();
    for (i, tm) in tms.enumerate() {
        let outcome = run_tm(&tm, args);

        summary.handle_outcome(outcome);
        let at_once = match N {
            1 | 2 => 1,
            3 => 1_000,
            4 => 100_000,
            _ => unreachable!(),
        };
        if !args.no_pb && i % at_once == 0 {
            pb.add(at_once as u64);
        }
    }

    if !args.no_pb {
        pb.finish();
        println!();
    }

    println!("  That took {:.2?}", before.elapsed());

    // ----- Print results ---------------------------------------------------
    println!();
    summary.print_report();
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

/// Simulate a turing machine.
fn run_tm<const N: usize>(tm: &Tm<{N}>, args: &Args) -> Outcome {
    // Before even running the TM, we analyze it to detect some non-terminating
    // TMs early.
    match static_analysis(tm) {
        Some(outcome) => return outcome,
        None => {}
    }


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
            return Outcome::AbortedAfterMaxSteps;
        }
    }


    let r = tape.written_range();
    let ones = (r.start.0..r.end.0).filter(|&id| tape.get(CellId(id)).0).count() as u64;

    Outcome::Halted { steps, ones }
}

fn static_analysis<const N: usize>(tm: &Tm<{N}>) -> Option<Outcome> {
    // Is the start 0 action transitioning to the halt state?
    if tm.states[0].on_0.will_halt() {
        return Some(Outcome::ImmediateHalt);
    }

    // Has the TM a transition to the halt state at all?
    if !tm.states.iter().flat_map(|s| s.actions()).any(|a| a.will_halt()) {
        return Some(Outcome::NoHaltState);
    }


    // Here we analyze whether we can even theoretically reach the halt state.
    // We do that by performing a depth-first search over the TM's states
    // (which form a graph). We use one additional trick: we first check if we
    // can reach a transition that can write a `1`. If that's not the case, we
    // can ignore all `on_1` transitions, meaning that this check will more
    // likely detect when a TM cannot halt.
    //
    // TODO: these `Vec`s are not necessary, we could use arrays.
    let mut stack = vec![0];
    let mut visited = vec![false; N];

    // Stays `true` until we encounter an action that actually writes a 1.
    let mut only_0s = true;

    let mut reached_halt = false;
    'outer: while let Some(state_id) = stack.pop() {
        if visited[state_id] {
            continue;
        }
        visited[state_id] = true;

        // Check if we could write a 1 from here.
        let state = &tm.states[state_id];
        if only_0s && state.on_0.write.0 {
            only_0s = false;

            // We have to reset the search here, because we ignored `on_1`
            // transitions so far. But since we can encounter 1s now, we have
            // to reconsider them again.
            stack = vec![0];
            visited = vec![false; N];
        }

        macro_rules! check_state {
            ($action:expr) => {
                if $action.will_halt() {
                    reached_halt = true;
                    break 'outer;
                }
                stack.push($action.next_state as usize);
            };
        }

        // If we haven't had the chance to write a 1 yet, we can ignore the
        // `on_1` transition.
        check_state!(state.on_0);
        if !only_0s {
            check_state!(state.on_1);
        }
    }

    if !reached_halt {
        return Some(Outcome::HaltStateNotReachable);
    }


    None
}
