use crate::{
    Outcome,
    tape::{CellId, Tape},
    tm::{Move, NextState, Tm},
};


/// Holds data used by different analysis operations. This is just a cache so
/// that we don't have to allocate memory again for each TM.
pub struct Analyzer<const N: usize> {
    max_steps: u32,

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

impl<const N: usize> Analyzer<N>
where
    [bool; N]: Default,
{
    /// Creates a new analyzer instance. You can use this instance to analyze
    /// different TMs. You can reuse this as often as you like. All values
    /// stored inside of this either don't change or are cleared for each new
    /// TM.
    pub fn new(max_steps: u32) -> Self {
        Self {
            max_steps,
            dfs_stack: Vec::new(),
            tape: Tape::new(),
        }
    }

    /// Main entry point: analyze the given TM.
    #[inline(never)]
    pub fn analyze(&mut self, tm: Tm<N>) -> Outcome {
        // Before even running the TM (dynamic analysis), we analyze it
        // statically to categorize certain TMs early.
        try_check!(Self::check_immediate_halt(tm));
        try_check!(Self::check_simple_elope(tm));
        // try_check!(Self::check_halt_exists(tm));
        // try_check!(self.check_halt_reachable(tm));

        self.run_tm(tm)
    }

    /// Static analysis (very fast): checks if the start 0 action is
    /// transitioning to the halt state. In that case the
    #[inline(always)]
    pub fn check_immediate_halt(tm: Tm<N>) -> Option<Outcome> {
        if tm.start_action().will_halt() {
            let wrote_one = tm.start_action().write_value().0;
            return Some(Outcome::ImmediateHalt { wrote_one });
        }

        None
    }

    /// Static analysis (very fast): checks if the first action has the start
    /// state as the next state. In those cases, the TM will just run away in
    /// one direction immediately.
    pub fn check_simple_elope(tm: Tm<N>) -> Option<Outcome> {
        if tm.start_action().next_state() == NextState::State(0) {
            return Some(Outcome::SimpleElope);
        }

        None
    }

    // /// Static analysis (fast): checks if the TM has a transition to the halt
    // /// state at all.
    // pub fn check_halt_exists(tm: Tm<N>) -> Option<Outcome> {
    //     use core::arch::x86_64::{_mm_movemask_epi8, _mm_set1_epi8, _mm_cmpeq_epi8, _mm_max_epu8};

    //     let v = *tm.encoded();
    //     unsafe {
    //         let mask = _mm_set1_epi8(0b1111_1100u8 as i8);

    //         // This first creates the maximum of v and mask, byte by byte. And
    //         // then compares to `v`. In the end, a byte in `compared` is `0xFF`
    //         // if the corresponding byte in v is equal to the max of v and
    //         // mask. This means that that byte in v was at least 1111_1100
    //         // large, meaning it contained the halt state. Otherwise the result
    //         // byte is 0.
    //         let compared = _mm_cmpeq_epi8(_mm_max_epu8(v, mask), v);

    //         // This compares v and mask byte by byte. If `v[i] < mask[i]`, the
    //         // result at that byte is `0xFF`, 0 otherwise. For unused bytes
    //         // (where no valid data is stored), `as_m128` returns zeroes. So
    //         // for those, the comparison is always true, always resulting in
    //         // `0xFF`.
    //         //
    //         // Note: the operands are in a strange order. They are swapped
    //         // basically. This is unfortunately intentional.
    //         // let compared = _mm_cmplt_epi8(mask, v);

    //         // With this, we extract the most significant bit of each byte. If
    //         // all of those are 0, that means there was no transition to the
    //         // halt state.
    //         if _mm_movemask_epi8(compared) == 0 {
    //             return Some(Outcome::NoHaltState);
    //         }
    //     }

    //     None
    // }

    // /// Static analysis (slower): check if the halt state can be reached via
    // /// the state graph.
    // ///
    // /// We do that by performing a depth-first search over the TM's states
    // /// (which form a graph). We use one additional trick: we first check if we
    // /// can reach a transition that can write a `1`. If that's not the case, we
    // /// can ignore all `on_1` transitions, meaning that this check will more
    // /// likely detect when a TM cannot halt.
    // #[inline(never)]
    // pub fn check_halt_reachable(&mut self, tm: Tm<N>) -> Option<Outcome> {
    //     let states = tm.states();

    //     self.dfs_stack.clear();
    //     self.dfs_stack.push(0);
    //     let mut visited: [bool; N] = array(false);

    //     // Stays `true` until we encounter an action that actually writes a 1.
    //     let mut only_0s = true;

    //     let mut reached_halt = false;
    //     'outer: while let Some(state_id) = self.dfs_stack.pop() {
    //         let state_visited = &mut visited[state_id as usize];
    //         if *state_visited {
    //             continue;
    //         }
    //         *state_visited = true;

    //         // Check if we could write a 1 from here.
    //         let state = &states[state_id as usize];
    //         if only_0s && state.on_0.write_value().0 {
    //             only_0s = false;

    //             // We have to reset the search here, because we ignored `on_1`
    //             // transitions so far. But since we can encounter 1s now, we have
    //             // to reconsider them again.
    //             self.dfs_stack.clear();
    //             self.dfs_stack.push(0);
    //             visited = array(false);
    //         }

    //         macro_rules! check_state {
    //             ($action:expr) => {
    //                 match $action.next_state() {
    //                     NextState::HaltState => {
    //                         reached_halt = true;
    //                         break 'outer;
    //                     }
    //                     NextState::State(v) => {
    //                         self.dfs_stack.push(v);
    //                     }
    //                 }
    //             };
    //         }

    //         // If we haven't had the chance to write a 1 yet, we can ignore the
    //         // `on_1` transition.
    //         check_state!(state.on_0);
    //         if !only_0s {
    //             check_state!(state.on_1);
    //         }
    //     }

    //     if !reached_halt {
    //         return Some(Outcome::HaltStateNotReachable);
    //     }

    //     None
    // }

    /// Actually run the TM.
    #[inline(never)]
    pub fn run_tm(&mut self, tm: Tm<N>) -> Outcome {
        self.tape.clear();
        let mut head = CellId(0);
        let mut current_state: u8 = 0;

        // The following variables are part of a simple run-away analysis.
        // Whenever we reach a cell outside of the "written range" (the range
        // spanning all cells there were written to), we set `running_away` to
        // `true`. We also mark the current state in `visited_during_run_away`.
        //
        // If we ever visit the same state twice during a run-away phase (i.e.
        // without returning into the "written range"), we know that the TM is
        // caught in a run-away loop. Attempt of a "proof":
        //
        // Assume we are currently in a run-away phase with the current state X
        // and we already visited X during this current run-away phase. Then
        // that means there is a loop from X to itself when using only `on_0`
        // transitions (in a run-away phase we only read 0 as cell values). And
        // since we are reading a 0 again, we will stay in that loop. Not that
        // within this loop we never got back inside of the "written range".
        // And this won't change, because the second time we encounter X the
        // `head` is *at least* as far outside of the "written range" as when
        // we first encountered X.
        let mut running_away = false;
        let mut visited_during_run_away: [bool; N] = array(false);

        let mut steps = 0;
        loop {
            steps += 1;

            if !self.tape.written_range().contains(&head) {
                running_away = true;
                let visited_state = &mut visited_during_run_away[current_state as usize];
                if *visited_state {
                    return Outcome::RunAwayDetected;
                } else {
                    *visited_state = true;
                }
            } else if running_away {
                // Reset everything related to this check.
                running_away = false;
                visited_during_run_away = array(false);
            }

            let value = self.tape.get(head);
            let action = tm.state(current_state).action_for(value);
            self.tape.write(head, action.write_value());

            current_state = match action.next_state() {
                NextState::HaltState => break,
                NextState::State(v) => v,
            };
            match action.movement() {
                Move::Left => head.0 -= 1,
                Move::Right => head.0 += 1,
            }

            if steps == self.max_steps {
                return Outcome::AbortedAfterMaxSteps;
            }
        }


        let r = self.tape.written_range();
        let ones = (r.start.0..r.end.0)
            .filter(|&id| self.tape.get(CellId(id)).0)
            .count() as u32;

        Outcome::Halted { steps, ones }
    }
}

fn array<T: Copy + Default, const N: usize>(v: T) -> [T; N]
where
    [T; N]: Default,
{
    let mut out: [T; N] = Default::default();
    out.iter_mut().for_each(|x| *x = v);
    out
}
