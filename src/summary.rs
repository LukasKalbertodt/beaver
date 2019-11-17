use std::cmp::min;

use term_painter::{ToStyle, Color::*};

use crate::{
    Args, Outcome,
};



pub struct Summary {
    /// The total number of TMs.
    num_tms: u64,

    /// The most number 1s written.
    high_score: u64,
    /// The number of TMs that have written `high_score` many 1s.
    num_winners: u64,
    /// The fewest number of steps a winner required to write `high_score` many
    /// 1s.
    fewest_winner_steps: u64,

    /// `Outcome::Halted`
    num_halted: u64,

    /// `Outcome::AbortedAfterMaxSteps`
    num_aborted_after_max_steps: u64,

    /// `Outcome::ImmediateHalt`
    num_immediate_halt: u64,

    /// `Outcome::NoHaltState`
    num_no_halt_state: u64,

    /// `Outcome::HaltStateNotReachable`
    num_halt_unreachable: u64,
}

impl Summary {
    pub fn new() -> Self {
        Self {
            num_tms: 0,
            high_score: 0,
            num_winners: 0,
            fewest_winner_steps: 0,
            num_halted: 0,
            num_aborted_after_max_steps: 0,
            num_immediate_halt: 0,
            num_no_halt_state: 0,
            num_halt_unreachable: 0,
        }
    }

    pub fn handle_outcome(&mut self, outcome: Outcome) {
        self.num_tms += 1;
        match outcome {
            Outcome::Halted { steps, ones } => {
                self.num_halted += 1;
                if ones > self.high_score {
                    self.high_score = ones;
                    self.num_winners = 1;
                    self.fewest_winner_steps = steps;
                } else if ones == self.high_score {
                    self.num_winners += 1;
                    self.fewest_winner_steps = min(self.fewest_winner_steps, steps);
                }
            }
            Outcome::AbortedAfterMaxSteps => self.num_aborted_after_max_steps += 1,
            Outcome::NoHaltState => self.num_no_halt_state += 1,
            Outcome::ImmediateHalt => self.num_immediate_halt += 1,
            Outcome::HaltStateNotReachable => self.num_halt_unreachable += 1,
        }
    }

    pub fn add(&mut self, other: Summary) {
        if self.high_score < other.high_score {
            self.high_score = other.high_score;
            self.num_winners = other.num_winners;
            self.fewest_winner_steps = other.fewest_winner_steps;
        }

        self.num_tms += other.num_tms;
        self.num_halted += other.num_halted;
        self.num_aborted_after_max_steps += other.num_aborted_after_max_steps;
        self.num_immediate_halt += other.num_immediate_halt;
        self.num_no_halt_state += other.num_no_halt_state;
        self.num_halt_unreachable += other.num_halt_unreachable;
    }

    fn percent(&self, v: u64) -> String {
        let percent = 100.0 * (v as f64) / (self.num_tms as f64);
        format!("{:.2}%", percent)
    }

    pub fn print_report(&self, args: &Args) {
        let halted_non_high_score = (self.num_halted + self.num_immediate_halt) - self.num_winners;
        let num_non_terminated =
            self.num_aborted_after_max_steps + self.num_no_halt_state + self.num_halt_unreachable;

        Blue.bold().with(|| println!("▸ Results:"));

        // ----- High scores
        println!(
            "- The high score (number of 1s after halting) is: {}",
            Green.bold().paint(self.high_score),
        );
        println!("  - {} TMs reached that high score", Green.bold().paint(self.num_winners));
        println!(
            "  - The quickest of which reached the high score in {} steps",
            Green.bold().paint(self.fewest_winner_steps),
        );

        // ----- Other halted TMs
        println!(
            "- {} ({}) TMs halted but did not get a high score",
            Yellow.bold().paint(halted_non_high_score),
            Yellow.bold().paint(self.percent(halted_non_high_score)),
        );
        println!(
            "  - {} ({}) TMs halted after 1 step (their first transition was to the halt state)",
            Yellow.bold().paint(self.num_immediate_halt),
            Yellow.bold().paint(self.percent(self.num_immediate_halt)),
        );

        // ----- Non-terminated
        println!(
            "- {} ({}) did not terminate:",
            Magenta.bold().paint(num_non_terminated),
            Magenta.bold().paint(self.percent(num_non_terminated)),
        );
        println!(
            "  - {} ({}) did not contain a transition to the halt state",
            Magenta.bold().paint(self.num_no_halt_state),
            Magenta.bold().paint(self.percent(self.num_no_halt_state)),
        );
        println!(
            "  - {} ({}) statically could not reach the halt state",
            Magenta.bold().paint(self.num_halt_unreachable),
            Magenta.bold().paint(self.percent(self.num_halt_unreachable)),
        );
        println!(
            "  - {} ({}) were aborted after the maximum number of steps ({})",
            Red.bold().paint(self.num_aborted_after_max_steps),
            Red.bold().paint(self.percent(self.num_aborted_after_max_steps)),
            args.max_steps,
        );
    }
}
