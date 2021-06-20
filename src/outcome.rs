
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
