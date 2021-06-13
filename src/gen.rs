//! Generating all turing machines with a given number of states.

use std::ops::Range;

use crate::tm::Tm;


/// Returns the number of different turing machines for a given N:
/// `(((N * 2 + 1) * 2)^2)^N`. For small N:
///
/// - 1: 36
/// - 2: 10_000
/// - 3: 7_529_536
/// - 4: 11_019_960_576
/// - 5: 26_559_922_791_424
pub fn num_machines(n: u64) -> u64 {
    (((n * 4) + 2).pow(2)).pow(n as u32)
}

#[inline(never)]
pub fn for_all_tms<F, const N: usize>(indices: Range<u64>, mut f: F)
where
    F: FnMut(Tm<N>),
    [(); 2 * N]:,
{
    use std::arch::x86_64::*;

    struct Consts {
        one: __m128i,
        halt_add: __m128i,
        clear_mask: __m128i,
    }

    // Create constant m128i values that are different for each "digit".
    let consts = {
        let mut indices = [0usize; 2 * N];
        for (i, e) in indices.iter_mut().enumerate() {
            *e = i;
        }

        macro_rules! shifted_vector {
            ($shift:expr, $yes:expr, $no:expr) => {
                _mm_set_epi8(
                    if $shift == 15 { $yes } else { $no },
                    if $shift == 14 { $yes } else { $no },
                    if $shift == 13 { $yes } else { $no },
                    if $shift == 12 { $yes } else { $no },
                    if $shift == 11 { $yes } else { $no },
                    if $shift == 10 { $yes } else { $no },
                    if $shift == 9 { $yes } else { $no },
                    if $shift == 8 { $yes } else { $no },
                    if $shift == 7 { $yes } else { $no },
                    if $shift == 6 { $yes } else { $no },
                    if $shift == 5 { $yes } else { $no },
                    if $shift == 4 { $yes } else { $no },
                    if $shift == 3 { $yes } else { $no },
                    if $shift == 2 { $yes } else { $no },
                    if $shift == 1 { $yes } else { $no },
                    if $shift == 0 { $yes } else { $no },
                )
            };
        }

        indices.map(|i| unsafe {
            Consts {
                one: shifted_vector!(i, 1, 0),
                // This is the difference between the first encoded action that
                // moves to the halting state (0xFC) and the last non-halt action.
                halt_add: shifted_vector!(i, (0xFC - (N * 4) + 1) as i8, 0),
                clear_mask: shifted_vector!(i, 0, 0xFFu8 as i8),
            }
        })
    };

    // This is an 2N digit number with base `4 * N + 2` (which is the number of
    // different possible states). `u8` is absolutely sufficient for all N that
    // one would realistically try.
    let base = 4 * N as u8 + 2;
    let mut counter = [0u8; 2 * N];

    unsafe {
        // Set the counter and tm corresponding to the start index.
        let mut tm = {
            let mut start = indices.start;
            for digit in &mut counter {
                *digit = (start % base as u64) as u8;
                start /= base as u64;
            }

            let bytes = counter.map(|digit| if digit < 4 * N as u8 {
                digit
            } else {
                0xFC + (digit - 4 * N as u8)
            });

            _mm_set_epi8(
                bytes.get(15).copied().unwrap_or(0) as i8,
                bytes.get(14).copied().unwrap_or(0) as i8,
                bytes.get(13).copied().unwrap_or(0) as i8,
                bytes.get(12).copied().unwrap_or(0) as i8,
                bytes.get(11).copied().unwrap_or(0) as i8,
                bytes.get(10).copied().unwrap_or(0) as i8,
                bytes.get(9).copied().unwrap_or(0) as i8,
                bytes.get(8).copied().unwrap_or(0) as i8,
                bytes.get(7).copied().unwrap_or(0) as i8,
                bytes.get(6).copied().unwrap_or(0) as i8,
                bytes.get(5).copied().unwrap_or(0) as i8,
                bytes.get(4).copied().unwrap_or(0) as i8,
                bytes.get(3).copied().unwrap_or(0) as i8,
                bytes.get(2).copied().unwrap_or(0) as i8,
                bytes.get(1).copied().unwrap_or(0) as i8,
                bytes.get(0).copied().unwrap_or(0) as i8,
            )
        };

        let count = indices.end - indices.start;
        'outer: for _ in 0..count {
            // Visit this TM.
            f(<Tm<N>>::new(tm));

            // Increment counter and adjust `tm`.
            for digit in 0..2 * N {
                counter[digit] += 1;
                if counter[digit] < 4 * N as u8 || counter[digit] == 4 * N as u8 + 1 {
                    tm = _mm_add_epi8(tm, consts[digit].one);
                } else if counter[digit] == 4 * N as u8 {
                    tm = _mm_add_epi8(tm, consts[digit].halt_add);
                } else if counter[digit] == base {
                    counter[digit] = 0;
                    tm = _mm_and_si128(tm, consts[digit].clear_mask);

                    // If we just incremented the most significant digit, we
                    // completely stop.
                    if digit + 1 == 2 * N {
                        break 'outer;
                    } else {
                        continue; // to next more significant digit
                    }
                }

                // If we have not reached the `else` above, there is no overflow
                // into the next digit and we stop.
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FULL_RANGE: Range<u64> = 0..u64::max_value();

    fn count<const N: usize>(range: Range<u64>) -> u64
    where
        [(); 2 * N]:
    {
        let mut count = 0;
        for_all_tms::<_, N>(range, |_| count += 1);
        count
    }

    fn to_vec<const N: usize>(range: Range<u64>) -> Vec<Tm<N>>
    where
        [(); 2 * N]:
    {
        let mut out = Vec::new();
        for_all_tms::<_, N>(range, |tm| out.push(tm));
        out
    }

    #[test]
    fn total_count_fits() {
        assert_eq!(count::<1>(FULL_RANGE), num_machines(1));
        assert_eq!(count::<2>(FULL_RANGE), num_machines(2));
        assert_eq!(count::<3>(FULL_RANGE), num_machines(3));
        // assert_eq!(count::<4>(FULL_RANGE), num_machines(4));
    }

    #[test]
    fn single_chunks_equal_full() {
        fn imp<const N: usize>()
        where
            [(); 2 * N]:
        {
            let all = to_vec::<N>(FULL_RANGE);
            for (i, tm) in all.iter().enumerate() {
                let single = to_vec::<N>(i as u64..i as u64 + 1);
                assert_eq!(&single, &[*tm]);
            }
        }

        imp::<1>();
        imp::<2>();
        imp::<3>();
        // imp::<4>();
    }
}
