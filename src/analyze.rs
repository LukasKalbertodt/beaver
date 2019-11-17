use std::{
    array::LengthAtMost32,
};

use crate::{
    Args, Outcome,
    tape::{CellId, Tape},
    tm::{Move, NextState, Tm},
};


/// Holds data used by different analysis operations. This is just a cache so
/// that we don't have to allocate memory again for each TM.
pub struct Analyzer<const N: usize> {
    args: Args,

    /// Stack containing state-ids used by `check_halt_reachable`.
    dfs_stack: Vec<u8>,

    /// The actual TM tape used by `run_tm`.
    tape: Tape,
}

macro_rules! try_check {
    ($e:expr) => {
        match $e {
            None => {}
            Some(outcome) => return outcome,
        }
    };
}

impl<const N: usize> Analyzer<{N}>
where
    [bool; N]: LengthAtMost32 + Default,
{
    /// Creates a new analyzer instance. You can use this instance to analyze
    /// different TMs. You can reuse this as often as you like. All values
    /// stored inside of this either don't change or are cleared for each new
    /// TM.
    pub fn new(args: Args) -> Self {
        Self {
            args,
            dfs_stack: Vec::new(),
            tape: Tape::new(),
        }
    }

    /// Main entry point: analyze the given TM.
    pub fn analyze(&mut self, tm: &Tm<{N}>) -> Outcome {
        // Before even running the TM (dynamic analysis), we analyze it
        // statically to categorize certain TMs early.
        try_check!(Self::check_immediate_halt(tm));
        try_check!(Self::check_simple_elope(tm));
        try_check!(Self::check_halt_exists(tm));
        try_check!(self.check_halt_reachable(tm));

        self.run_tm(tm)
    }

    /// Static analysis (very fast): checks if the start 0 action is
    /// transitioning to the halt state. In that case the
    #[inline(always)]
    pub fn check_immediate_halt(tm: &Tm<{N}>) -> Option<Outcome> {
        if tm.start_action().will_halt() {
            let wrote_one = tm.start_action().write_value().0;
            return Some(Outcome::ImmediateHalt { wrote_one });
        }

        None
    }

    /// Static analysis (very fast): checks if the first action has the start
    /// state as the next state. In those cases, the TM will just run away in
    /// one direction immediately.
    pub fn check_simple_elope(tm: &Tm<{N}>) -> Option<Outcome> {
        if tm.start_action().next_state() == NextState::State(0) {
            return Some(Outcome::SimpleElope);
        }

        None
    }

    /// Static analysis (fast): checks if the TM has a transition to the halt
    /// state at all.
    #[inline(always)]
    pub fn check_halt_exists(tm: &Tm<{N}>) -> Option<Outcome> {
        if !tm.states.iter().flat_map(|s| s.actions()).any(|a| a.will_halt()) {
            return Some(Outcome::NoHaltState);
        }

        None
    }

    /// Static analysis (slower): check if the halt state can be reached via
    /// the state graph.
    ///
    /// We do that by performing a depth-first search over the TM's states
    /// (which form a graph). We use one additional trick: we first check if we
    /// can reach a transition that can write a `1`. If that's not the case, we
    /// can ignore all `on_1` transitions, meaning that this check will more
    /// likely detect when a TM cannot halt.
    #[inline(never)]
    pub fn check_halt_reachable(&mut self, tm: &Tm<{N}>) -> Option<Outcome> {
        self.dfs_stack.clear();
        self.dfs_stack.push(0);
        let mut visited: [bool; N] = array(false);

        // Stays `true` until we encounter an action that actually writes a 1.
        let mut only_0s = true;

        let mut reached_halt = false;
        'outer: while let Some(state_id) = self.dfs_stack.pop() {
            let state_visited = &mut visited[state_id as usize];
            if *state_visited {
                continue;
            }
            *state_visited = true;

            // Check if we could write a 1 from here.
            let state = &tm.states[state_id as usize];
            if only_0s && state.on_0.write_value().0 {
                only_0s = false;

                // We have to reset the search here, because we ignored `on_1`
                // transitions so far. But since we can encounter 1s now, we have
                // to reconsider them again.
                self.dfs_stack.clear();
                self.dfs_stack.push(0);
                visited = array(false);
            }

            macro_rules! check_state {
                ($action:expr) => {
                    match $action.next_state() {
                        NextState::HaltState => {
                            reached_halt = true;
                            break 'outer;
                        }
                        NextState::State(v) => {
                            self.dfs_stack.push(v);
                        }
                    }
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

    /// Actually run the TM.
    #[inline(never)]
    pub fn run_tm(&mut self, tm: &Tm<{N}>) -> Outcome {
        self.tape.clear();
        let mut head = CellId(0);
        let mut current_state: u8 = 0;

        let mut steps = 0;
        loop {
            steps += 1;

            let value = self.tape.get(head);
            let action = tm.states[current_state as usize].action_for(value);
            self.tape.write(head, action.write_value());

            current_state = match action.next_state() {
                NextState::HaltState => break,
                NextState::State(v) => v,
            };
            match action.movement() {
                Move::Left => head.0 -= 1,
                Move::Right => head.0 += 1,
            }

            if steps == self.args.max_steps {
                return Outcome::AbortedAfterMaxSteps;
            }
        }


        let r = self.tape.written_range();
        let ones = (r.start.0..r.end.0).filter(|&id| self.tape.get(CellId(id)).0).count() as u64;

        Outcome::Halted { steps, ones }
    }
}

fn array<T: Copy + Default, const N: usize>(v: T) -> [T; {N}]
where
    [T; N]: Default,
{
    let mut out: [T; N] = Default::default();
    out.iter_mut().for_each(|x| *x = v);
    out
}
