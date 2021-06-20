use std::ops::Range;

use crate::tm::Tm;

#[cfg(test)]
#[macro_use]
mod tests;

mod all;

pub use self::{
    all::All,
};

/// Something that can generate N state Turing machines.
pub trait TmGenerator<const N: usize> {
    fn description() -> &'static str;

    /// The number of different Turing machines this generator can generate in
    /// total.
    fn num_tms() -> u64;

    /// The number of different actions that Turing machines returned by this
    /// generated can have.
    fn num_possible_actions() -> u64;

    /// Returns one specific TM for an index between 0 and `Self::num_tms
    /// ()`. Note that this is an index in the arbitrary (but fixed) order in
    /// which this generator generates TMs, and NOT the ID of the TM.
    fn tm_at(index: u64) -> Tm<N>;

    /// Generates all TMs in the given range of indices (not TM IDs!).
    fn for_range<F: FnMut(Tm<N>)>(range: Range<u64>, f: F);

    /// Generates all TMs this generate can generate.
    fn for_all<F: FnMut(Tm<N>)>(f: F) {
        Self::for_range(0..Self::num_tms(), f)
    }
}
