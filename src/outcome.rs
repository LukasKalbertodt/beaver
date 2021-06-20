/// A sink that accumulates results from analyzing TMs. One method per different
/// analysis result.
pub trait OutcomeSink {
    /// The TM ran and halted.
    fn report_halted(&mut self, num_steps: u32, num_ones: u32);

    /// The start state of the TM for the cell value 0 has the halt state as
    /// next state. This means the TM terminates in one step. It might write a
    /// single one, though.
    fn report_immediate_halt(&mut self, wrote_one: bool);

    /// The TM does not even have a transition to the halt state at all.
    fn report_no_halt_transition(&mut self);

    /// The TM does immediately go into one direction without ever stopping.
    /// This happens if the start action has `next_state == 0`.
    fn report_simple_elope(&mut self);

    /// If the turing machine has a state graph where the halt state cannot be
    /// reached from the start state.
    fn report_halt_state_not_reachable(&mut self);

    /// While executing the TM a run-away was detected, meaning that the TM
    /// was caught in a loop only visiting new cells, thus never terminating.
    fn report_run_away(&mut self);

    /// The TM ran but was aborted after the maximum number of steps.
    fn report_aborted_after_max_steps(&mut self);
}
