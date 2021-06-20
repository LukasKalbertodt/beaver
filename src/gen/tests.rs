

macro_rules! gen_tests {
    ($gen:ident) => {
        use super::*;

        #[test]
        fn total_count_fits() {
            fn count_all<const N: usize>() -> u64 {
                let mut count = 0;
                <$gen<N>>::for_all(|_| count += 1);
                count
            }

            assert_eq!(count_all::<1>(), <$gen<1>>::num_tms());
            assert_eq!(count_all::<2>(), <$gen<2>>::num_tms());
            assert_eq!(count_all::<3>(), <$gen<3>>::num_tms());
            // assert_eq!(count::<4>(FULL_RANGE), num_machines(4));
        }

        #[test]
        fn chunked_equals_full() {
            fn imp<const N: usize>() {
                let mut all = Vec::new();
                <$gen<N>>::for_all(|tm| all.push(tm));

                let chunk_size = match N {
                    1 => 5,
                    2 => 80,
                    _ => 10_000,
                };

                let mut chunked = Vec::new();
                for start in (0..<$gen<N>>::num_tms()).step_by(chunk_size as usize) {
                    let end = std::cmp::min(<$gen<N>>::num_tms(), start + chunk_size);
                    <$gen<N>>::for_range(start..end, |tm| chunked.push(tm));
                }

                assert_eq!(all, chunked);
            }

            imp::<1>();
            imp::<2>();
            imp::<3>();
            // imp::<4>();
        }

        #[test]
        fn single_tm_equals_ranged() {
            fn imp<const N: usize>() {
                let mut all = Vec::new();
                <$gen<N>>::for_all(|tm| all.push(tm));

                for index in 0..<$gen<N>>::num_tms() {
                    assert_eq!(all[index as usize], <$gen<N>>::tm_at(index));
                }
            }

            imp::<1>();
            imp::<2>();
            imp::<3>();
            // imp::<4>();
        }

        #[test]
        fn all_unique() {
            fn imp<const N: usize>() {
                let mut all = Vec::new();
                <$gen<N>>::for_all(|tm| all.push(tm.encoded));

                all.sort();
                assert!((0..all.len() - 1).all(|i| all[i] != all[i + 1]));
            }

            imp::<1>();
            imp::<2>();
            imp::<3>();
            // imp::<4>();
        }
    };
}
