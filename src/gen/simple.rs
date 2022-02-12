use std::ops::Range;

use crate::tm::Tm;

use super::TmGenerator;


/// This is a pretty generic generator whose behavior can be configured via the
/// const parameter `OPTS`.
pub struct Gen<const OPTS: Opt, const N: usize>;

#[derive(PartialEq, Eq)]
pub enum Opt {
    /// No optimization, every single possible TM is yielded.
    None,

    /// This skips one TM per pair of TMs that are symmetric to one another.
    /// There are two symmetries used here:
    ///
    /// 1. For any TM you could flip all direction bits of all transitions and
    /// get a new TM that will behave exactly like the first one, but with the
    /// tape mirrored. The symmetries are skipped here by only yielding TMs
    /// whose last state's `on_1` transition moves left.
    ///
    /// 2. Another "symmetry" is found by looking at the head move direction of
    /// transitions to the halt state. The direction here does not matter to us
    /// at all. So the generator would only yield TMs where all transitions to
    /// the halt state move the head left.
    SkipSymmetries,

    /// In addtion to skipping symmetries, this also does not yield any TMs
    /// where a transition to the halt state writes 0 to the tape. This *does*
    /// change the histogram and the distribution of TMs! It is not as clean of
    /// an optimization as `SkipSymmetries` alone. However, TMs that write 0
    /// before transitioning to HALT are not interesting when trying to find
    /// busy beavers, since the same TM, but writing a 1 instead, would always
    /// perform better.
    AlsoSkipHaltZero,
}


pub type All<const N: usize> = Gen<{ Opt::None }, N>;
pub type NoSymmetries<const N: usize> = Gen<{ Opt::SkipSymmetries }, N>;
pub type Optimized<const N: usize> = Gen<{ Opt::AlsoSkipHaltZero }, N>;

impl<const OPTS: Opt, const N: usize> TmGenerator<N> for Gen<OPTS, N> {
    fn description() -> &'static str {
        match OPTS {
            Opt::None => "All TMs",
            Opt::SkipSymmetries => "All TMs but symmetric pairs deduplicated",
            Opt::AlsoSkipHaltZero
                => "All TMs without symmetry and without TMs with H_0 transitions",
        }
    }

    fn num_tms() -> u64 {
        // There are N states and 2 actions per state.
        let mut out = Self::num_possible_actions().pow(2 * N as u32);

        // See symmetry (1) in the type docs. This cuts the number of TMs in
        // half.
        if OPTS != Opt::None {
            out /= 2;
        }

        out
    }

    fn num_possible_actions() -> u64 {
        // These are `N + 1` (due to halt state) possible new states, each with
        // two different values to write and two different directions to move.
        let out = (1 + N as u64) * 2 * 2;

        match OPTS {
            Opt::None => out,

            // Skipping symmetries here means that we ignore transitions to the
            // halt state that move right. Thanks to our TM encoding, these are
            // simply the last two possible 5 bit values of one action. The
            // last four actions are: Hl1, Hl0, Hr1, Hr0.
            Opt::SkipSymmetries => out - 2,

            // Similarly, if we also want to skip all halt transitions that
            // write 0, we just stop one earlier still.
            Opt::AlsoSkipHaltZero => out - 3,
        }
    }

    fn tm_at(mut index: u64) -> Tm<N> {
        assert!(index < Self::num_tms());

        let mut out = 0;
        for i in 0..2 * N {
            let action = index % Self::num_possible_actions();
            out |= action << (i * 5);
            index /= Self::num_possible_actions();
        }

        // If we skip symmetries, the calculated value has to be adjusted.
        if OPTS != Opt::None {
            // The highest action (`on_1` on last state) is incorrect because
            // the 2nd bit can be 0 or 1. But it needs to always be 0. On the
            // other hand, since `Self::num_tms()` is half of the value we had
            // without the optimization, we are missing half the possible
            // actions. The solution is actually quite easy: we just take all
            // bits above and including the bit of the direction (which should
            // be 0) and shift them left by one. You can also imagine inserting
            // a 0 bit, pushing the other bits to the left.
            //
            // Consider these 10 possible actions for N=2:
            //
            //  1: Al1  0000     | shifted:  0000 -> Al1
            //  2: Al0  0001     | shifted:  0001 -> Al0
            //  3: Ar1  0010     | shifted:  0100 -> Bl1
            //  4: Ar0  0011     | shifted:  0101 -> Bl0
            //  5: Bl1  0100     | shifted:  1000 -> Hl1
            //  6: Bl0  0101
            //  7: Br1  0110
            //  8: Br0  0111
            //  9: Hl1  1000
            // 10: Hl0  1001
            //
            // As you can see, when we only take the first half, but then insert
            // the 0 bit at the second bit position, pushing everything else to
            // the left, we magically get the actions we want.

            let bad_bit_pos = 5 * (2 * N - 1) + 1;
            let lower_mask = (1 << bad_bit_pos) - 1;
            let lower = out & lower_mask;
            let upper = out & !lower_mask;
            out = (upper << 1) | lower;
        }

        Tm::new_unchecked(out)
    }

    fn for_range<F: FnMut(Tm<N>)>(range: Range<u64>, mut f: F) {
        assert!(range.end <= Self::num_tms());

        let mut current = Self::tm_at(range.start).encoded;
        for _ in range {
            f(<Tm<N>>::new_unchecked(current));

            for i in 0..2 * N {
                let offset = 5 * i;

                // Increment by 1 (generally speaking)
                if i + 1 == 2 * N && OPTS != Opt::None {
                    // If this is the last digit AND the "skip symmetry"
                    // optimization is enabled, we need to skip actions that
                    // move right here. The second to last bit is always 0 due
                    // to the `if` in `tm_at` and due to what's following.
                    if (current >> offset) & 0b1 == 0 {
                        current += 1 << offset;
                    } else {
                        // Add 1 and skip 2. E.g. from `b00001` to `b00100`.
                        current += 3 << offset;
                    }
                } else {
                    current += 1 << offset;
                }

                // Check for overflow in this digit. If so, we continue to carry and
                // set this digit to 0.
                if (current >> offset) & 0b11111 == Self::num_possible_actions() {
                    // If we are at the last digit and have a carry, we are done.
                    if i + 1 == 2 * N {
                        return;
                    }

                    current &= !(0b11111 << offset);
                } else {
                    // Otherwise we are done advancing the iterator.
                    break;
                }
            }
        }
    }
}


#[cfg(test)]
mod tests_all {
    gen_tests!(All);
}

#[cfg(test)]
mod tests_without_symmetries {
    gen_tests!(NoSymmetries);
}

#[cfg(test)]
mod tests_optimized {
    gen_tests!(Optimized);
}
