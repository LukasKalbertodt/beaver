use std::ops::Range;

use crate::tm::Tm;

use super::TmGenerator;


pub struct All<const N: usize>;

impl<const N: usize> TmGenerator<N> for All<N> {
    fn num_tms() -> u64 {
        // There are N states and 2 actions per state.
        Self::num_possible_actions().pow(2 * N as u32)
    }

    fn num_possible_actions() -> u64 {
        // Since this generator returns absolutely all possible TMs, we are not
        // clever at all and just use all actions, even quite useless ones. So
        // we end up with `N + 1` (due to halt state) possible new states, each
        // with two different values to write and two different directions to
        // move.
        (1 + N as u64) * 2 * 2
    }

    fn tm_at(mut index: u64) -> Tm<N> {
        assert!(index < Self::num_tms());

        let mut out = 0;
        for i in 0..2 * N {
            let action = index % Self::num_possible_actions();
            out |= action << (i * 5);
            index /= Self::num_possible_actions();
        }

        Tm::new_unchecked(out)
    }

    fn for_range<F: FnMut(Tm<N>)>(range: Range<u64>, mut f: F) {
        let mut current = Self::tm_at(range.start).encoded;
        for _ in range {
            f(<Tm<N>>::new_unchecked(current));

            for i in 0..2 * N {
                let offset = 5 * i;

                // Increment by 1
                current += 1 << offset;

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
mod tests {
    gen_tests!(All);
}
