pub trait PrimalityTest {
    fn is_prime(&self, n: u64) -> bool;
}

pub mod primality_test_algorithms {
    pub enum DivisionUpperBound {
        Whole,
        Half,
        Square,
    }

    pub struct NaiveTrialDivision {
        pub increment: u64,
        pub upper_bound: DivisionUpperBound,
    }

    impl super::PrimalityTest for NaiveTrialDivision {
        fn is_prime(&self, n: u64) -> bool {
            if n == 2 || n == 3 {
                return true;
            }

            if n <= 1 || n % 2 == 0 {
                return false;
            }

            let mut i = 3;
            let upper_bound = match self.upper_bound {
                DivisionUpperBound::Whole => n,
                DivisionUpperBound::Half => n / 2 + 1,
                DivisionUpperBound::Square => (n as f64).sqrt() as u64 + 1,
            };

            while i < upper_bound && n % i != 0 {
                i = i + self.increment
            }

            i >= upper_bound
        }
    }

    pub struct SixKOneDivision {
        pub upper_bound: DivisionUpperBound,
    }

    impl super::PrimalityTest for SixKOneDivision {
        fn is_prime(&self, n: u64) -> bool {
            if n == 2 || n == 3 {
                return true;
            }

            if n <= 1 || n % 2 == 0 || n % 3 == 0 {
                return false;
            }

            let upper_bound = match self.upper_bound {
                DivisionUpperBound::Whole => n,
                DivisionUpperBound::Half => n / 2 + 1,
                DivisionUpperBound::Square => (n as f64).sqrt() as u64 + 1,
            };

            let mut k1 = 5;
            let mut k2 = 7;

            while (k1 <= upper_bound || k2 <= upper_bound)
                && n % k1 != 0
                && n % k2 != 0
            {
                k1 = k1 + 6;
                k2 = k2 + 6;
            }

            if k1 < upper_bound && k2 < upper_bound {
                return false;
            }

            return n % k1 == 0
                || n % k2 == 0
                || (k1 >= upper_bound && k2 >= upper_bound);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::primality_test_algorithms::*;
    use super::*;

    const PRIMES_TO_100: &[u64] = &[
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67,
        71, 73, 79, 83, 89, 97,
    ];

    const NOT_PRIMES: &[u64] = &[
        0, 1, 4, 6, 8, 9, 10, 12, 14, 15, 16, 18, 20, 21, 22, 24, 25, 26, 27,
        28, 30, 32, 33, 34, 35,
    ];

    #[test]
    fn test_whole_naive_trial_division_increment_by_one() {
        let ntd = NaiveTrialDivision {
            increment: 1,
            upper_bound: DivisionUpperBound::Whole,
        };

        for p in PRIMES_TO_100 {
            assert_eq!(ntd.is_prime(*p), true);
        }

        for p in NOT_PRIMES {
            assert_eq!(ntd.is_prime(*p), false);
        }
    }

    #[test]
    fn test_whole_naive_trial_division_increment_by_two() {
        let ntd = NaiveTrialDivision {
            increment: 2,
            upper_bound: DivisionUpperBound::Whole,
        };

        for p in PRIMES_TO_100 {
            assert_eq!(ntd.is_prime(*p), true);
        }

        for p in NOT_PRIMES {
            assert_eq!(ntd.is_prime(*p), false);
        }
    }

    #[test]
    fn test_half_naive_trial_division_increment_by_one() {
        let ntd = NaiveTrialDivision {
            increment: 1,
            upper_bound: DivisionUpperBound::Half,
        };

        for p in PRIMES_TO_100 {
            assert_eq!(ntd.is_prime(*p), true);
        }

        for p in NOT_PRIMES {
            assert_eq!(ntd.is_prime(*p), false);
        }
    }

    #[test]
    fn test_half_naive_trial_division_increment_by_two() {
        let ntd = NaiveTrialDivision {
            increment: 2,
            upper_bound: DivisionUpperBound::Half,
        };

        for p in PRIMES_TO_100 {
            assert_eq!(ntd.is_prime(*p), true);
        }

        for p in NOT_PRIMES {
            assert_eq!(ntd.is_prime(*p), false);
        }
    }

    #[test]
    fn test_square_naive_trial_division_increment_by_one() {
        let ntd = NaiveTrialDivision {
            increment: 1,
            upper_bound: DivisionUpperBound::Square,
        };

        for p in PRIMES_TO_100 {
            assert_eq!(ntd.is_prime(*p), true);
        }

        for p in NOT_PRIMES {
            assert_eq!(ntd.is_prime(*p), false);
        }
    }

    #[test]
    fn test_square_naive_trial_division_increment_by_two() {
        let ntd = NaiveTrialDivision {
            increment: 2,
            upper_bound: DivisionUpperBound::Square,
        };

        for p in PRIMES_TO_100 {
            assert_eq!(ntd.is_prime(*p), true);
        }

        for p in NOT_PRIMES {
            assert_eq!(ntd.is_prime(*p), false);
        }
    }

    #[test]
    fn test_whole_six_k_one_division() {
        let skod = SixKOneDivision {
            upper_bound: DivisionUpperBound::Whole,
        };

        for p in PRIMES_TO_100 {
            assert_eq!(skod.is_prime(*p), true);
        }

        for p in NOT_PRIMES {
            assert_eq!(skod.is_prime(*p), false);
        }
    }

    #[test]
    fn test_half_six_k_one_division() {
        let skod = SixKOneDivision {
            upper_bound: DivisionUpperBound::Half,
        };

        for p in PRIMES_TO_100 {
            assert_eq!(skod.is_prime(*p), true);
        }

        for p in NOT_PRIMES {
            assert_eq!(skod.is_prime(*p), false);
        }
    }

    #[test]
    fn test_square_six_k_one_division() {
        let skod = SixKOneDivision {
            upper_bound: DivisionUpperBound::Square,
        };

        for p in PRIMES_TO_100 {
            assert_eq!(skod.is_prime(*p), true);
        }

        for p in NOT_PRIMES {
            assert_eq!(skod.is_prime(*p), false);
        }
    }
}
