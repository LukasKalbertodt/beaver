use std::{
    cmp::min,
    collections::HashMap,
};

use crate::Outcome;



pub struct Summary {
    /// The total number of TMs.
    num_tms: u64,

    /// The most number 1s written.
    high_score: u32,

    /// The number of TMs that have written `high_score` many 1s.
    num_winners: u64,

    /// The fewest number of steps a winner required to write `high_score` many
    /// 1s.
    fewest_winner_steps: u32,

    /// Records how many TMs finished after how many steps.
    step_histogram: HashMap<u32, u64>,

    /// `Outcome::Halted`
    num_halted: u64,

    /// `Outcome::AbortedAfterMaxSteps`
    num_aborted_after_max_steps: u64,

    /// `Outcome::ImmediateHalt`
    num_immediate_halt: u64,

    /// `Outcome::SimpleElope`
    num_simple_elope: u64,

    /// `Outcome::NoHaltState`
    num_no_halt_state: u64,

    /// `Outcome::HaltStateNotReachable`
    num_halt_unreachable: u64,

    /// `Outcome::RunAwayDetected` (dynamic analysis)
    num_runaway_dyn: u64,
}

impl Summary {
    pub fn new() -> Self {
        Self {
            num_tms: 0,
            high_score: 0,
            num_winners: 0,
            fewest_winner_steps: 0,
            step_histogram: HashMap::new(),
            num_halted: 0,
            num_aborted_after_max_steps: 0,
            num_immediate_halt: 0,
            num_simple_elope: 0,
            num_no_halt_state: 0,
            num_halt_unreachable: 0,
            num_runaway_dyn: 0,
        }
    }

    fn handle_high_score(&mut self, ones: u32, steps: u32) {
        if ones > self.high_score {
            self.high_score = ones;
            self.num_winners = 1;
            self.fewest_winner_steps = steps;
        } else if ones == self.high_score {
            self.num_winners += 1;
            self.fewest_winner_steps = min(self.fewest_winner_steps, steps);
        }
    }

    pub fn handle_outcome(&mut self, outcome: Outcome) {
        self.num_tms += 1;
        match outcome {
            Outcome::Halted { steps, ones } => {
                self.num_halted += 1;
                self.handle_high_score(ones, steps);

                *self.step_histogram.entry(steps).or_insert(0) += 1;
            }
            Outcome::ImmediateHalt { wrote_one } => {
                self.num_immediate_halt += 1;
                if wrote_one {
                    self.handle_high_score(1, 1);
                }
                *self.step_histogram.entry(1).or_insert(0) += 1;
            }
            Outcome::AbortedAfterMaxSteps => self.num_aborted_after_max_steps += 1,
            Outcome::SimpleElope => self.num_simple_elope += 1,
            Outcome::NoHaltState => self.num_no_halt_state += 1,
            Outcome::HaltStateNotReachable => self.num_halt_unreachable += 1,
            Outcome::RunAwayDetected => self.num_runaway_dyn += 1,
        }
    }

    pub fn add(&mut self, other: Summary) {
        if self.high_score < other.high_score {
            self.high_score = other.high_score;
            self.num_winners = other.num_winners;
            self.fewest_winner_steps = other.fewest_winner_steps;
        } else if self.high_score == other.high_score {
            self.num_winners += other.num_winners;
            self.fewest_winner_steps = min(self.fewest_winner_steps, other.fewest_winner_steps);
        }

        self.num_tms += other.num_tms;
        self.num_halted += other.num_halted;
        self.num_aborted_after_max_steps += other.num_aborted_after_max_steps;
        self.num_immediate_halt += other.num_immediate_halt;
        self.num_simple_elope += other.num_simple_elope;
        self.num_no_halt_state += other.num_no_halt_state;
        self.num_halt_unreachable += other.num_halt_unreachable;
        self.num_runaway_dyn += other.num_runaway_dyn;

        for (steps, count) in &other.step_histogram {
            *self.step_histogram.entry(*steps).or_insert(0) += count;
        }
    }

    fn percent(&self, v: u64) -> String {
        let percent = 100.0 * (v as f64) / (self.num_tms as f64);
        format!("{:.2}%", percent)
    }

    pub fn print_report(&self, args: &super::Args) {
        let halted_non_high_score = (self.num_halted + self.num_immediate_halt) - self.num_winners;
        let num_non_terminated = self.num_aborted_after_max_steps
            + self.num_simple_elope
            + self.num_no_halt_state
            + self.num_halt_unreachable
            + self.num_runaway_dyn;

        bunt::println!("{$blue+bold}▸ Results:{/$}");

        // ----- High scores
        bunt::println!(
            "- The high score (number of 1s after halting) is: {[green+bold]}",
            self.high_score,
        );
        bunt::println!("  - {[green+bold]} TMs reached that high score", self.num_winners);
        bunt::println!(
            "  - The quickest of which reached the high score in {[bold+green]} steps",
            self.fewest_winner_steps,
        );

        // ----- Other halted TMs
        bunt::println!(
            "- {[yellow+bold]} ({[yellow+bold]}) TMs halted but did not get a high score",
            halted_non_high_score,
            self.percent(halted_non_high_score),
        );
        bunt::println!(
            "  - {[yellow+bold]} ({[yellow+bold]}) TMs halted after 1 step \
                (their first transition was to the halt state)",
            self.num_immediate_halt,
            self.percent(self.num_immediate_halt),
        );

        // ----- Non-terminated
        bunt::println!(
            "- {[magenta+bold]} ({[magenta+bold]}) did not terminate:",
            num_non_terminated,
            self.percent(num_non_terminated),
        );
        bunt::println!(
            "  - {[magenta+bold]} ({[magenta+bold]}) immediately ran away in one direction \
                and remained in the start state",
            self.num_simple_elope,
            self.percent(self.num_simple_elope),
        );
        bunt::println!(
            "  - {[magenta+bold]} ({[magenta+bold]}) did not contain a transition \
                to the halt state",
            self.num_no_halt_state,
            self.percent(self.num_no_halt_state),
        );
        bunt::println!(
            "  - {[magenta+bold]} ({[magenta+bold]}) statically could not reach the halt state",
            self.num_halt_unreachable,
            self.percent(self.num_halt_unreachable),
        );
        bunt::println!(
            "  - {[magenta+bold]} ({[magenta+bold]}) were caught in a run-away loop",
            self.num_runaway_dyn,
            self.percent(self.num_runaway_dyn),
        );
        bunt::println!(
            "  - {[red+bold]} ({[red+bold]}) were aborted after the maximum number of steps ({})",
            self.num_aborted_after_max_steps,
            self.percent(self.num_aborted_after_max_steps),
            args.shared.max_steps,
        );

        println!();
        let gcd = gcd(&[
            self.num_winners,
            halted_non_high_score,
            self.num_immediate_halt,
            self.num_simple_elope,
            self.num_no_halt_state,
            self.num_halt_unreachable,
            self.num_runaway_dyn,
            self.num_aborted_after_max_steps,
        ]);
        println!("Hint: the greatest common denominator of all these numbers is {}.", gcd);
        if gcd == 1 {
            println!("This means the chosen generator does not generate duplicate/equivalent \
                TMs. That's good!");
        } else {
            println!("This means the chosen generator generates {}-tuples of TMs that are \
                equivalent to one another.", gcd);
            println!("A better generator could speed this up by eleminating duplicate TMs.");
        };
        println!();

        if !args.hide_histogram {
            println!();
            println!();
            self.print_histogram(args);
        }

        println!();
    }

    fn print_histogram(&self, args: &super::Args) {
        let histogram_height = args.histogram_height as usize;
        let histogram_cutoff = args.histogram_cutoff;

        if self.step_histogram.is_empty() {
            println!("   (histogram not shown as TMs ran for at most 1 step)");
            return;
        } else {
            bunt::println!("{$blue+bold}▸ Histogram (how many TMs halted after x steps):{/$}");
            println!("note: the y-axis is logarithmic");
            println!();
        }

        let max = self.step_histogram.values().max().copied().expect("histogram empty");
        let max_log = (max as f64).log10();

        let mut lines = vec![String::new(); histogram_height];
        for (row, line) in lines.iter_mut().enumerate() {
            let inv_row = histogram_height - row - 1;
            if inv_row == 0 {
                line.push_str("        0 ▕");
            } else if inv_row == histogram_height - 1 {
                line.push_str(&format!("{: >9} ▕", max));
            } else if inv_row % 2 == 0 && inv_row < histogram_height - 2 {
                let ratio = inv_row as f64 / histogram_height as f64;
                let v = 10.0f64.powf(max_log * ratio).round() as u32;
                line.push_str(&format!("{: >9} ▕", v));
            } else {
                line.push_str("          ▕");
            }
        }

        for steps in 1..histogram_cutoff {
            lines[..histogram_height - 1].iter_mut().for_each(|l| l.push(' '));
            lines[histogram_height - 1].push('▁');

            let count = self.step_histogram.get(&steps).copied().unwrap_or(0);
            let count_log = if count == 0 {
                0.0
            } else {
                (count as f64).log10()
            };

            // This is the height of the bar in eigths steps. We have to offset
            // it all by 1 since we need the lowest eigth for the bottom line.
            // To make logic inside the loop easier this height we are
            // calculating is at least 1. In other words: now the bottom line
            // is part of the bar.
            let bar_height_eights = (
                (8.0 * (histogram_height as f64) - 1.0) * (count_log / max_log)
            ).round() as usize + 1;

            for row in 0..histogram_height {
                let inv_row = histogram_height - row - 1;
                let eights_in_this_line = std::cmp::min(
                    bar_height_eights.saturating_sub(inv_row * 8),
                    8,
                );
                let symbols = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
                let symbol = symbols[eights_in_this_line];
                lines[row].push(symbol);
                lines[row].push(symbol);
            }
        }

        lines.iter().for_each(|l| println!("{}", l));

        print!("    steps: ");
        for steps in 1..histogram_cutoff {
            print!("{: >3}", steps);
        }
        println!();

        print!("    count: ");
        for steps in 1..histogram_cutoff {
            let count = self.step_histogram.get(&steps).copied().unwrap_or(0);
            if count < 100 {
                print!(" {: >2}", count);
            } else {
                print!("   ");
            }
        }
        println!();
    }
}

/// Returns the greatest common denominator of all given numbers.
fn gcd(nums: &[u64]) -> u64 {
    let mut gdc = nums[0];
    for mut n in nums[1..].iter().copied() {
        while n > 0 {
            let tmp = n;
            n = gdc % n;
            gdc = tmp;
        }
    }

    gdc
}
