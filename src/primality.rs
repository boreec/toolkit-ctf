pub trait PrimalityTest {
    fn is_prime(n: u64) -> bool;
}

pub mod primality_test_algorithms {
    pub struct NaiveDivision;

    impl super::PrimalityTest for NaiveDivision {
        fn is_prime(n: u64) -> bool {
            if n <= 1 {
                return false;
            }
            if n == 2 {
                return true;
            }

            for i in 2..n {
                if n % i == 0 {
                    return false;
                }
            }

            return true;
        }
    }

    pub struct NaiveHalfDivision;

    impl super::PrimalityTest for NaiveHalfDivision {
        fn is_prime(n: u64) -> bool {
            if n <= 1 {
                return false;
            }
            if n == 2 || n == 3 {
                return true;
            }

            for i in 2..=(n / 2) {
                if n % i == 0 {
                    return false;
                }
            }

            return true;
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
    fn test_naive_division() {
        for p in PRIMES_TO_100 {
            assert_eq!(NaiveDivision::is_prime(*p), true);
        }

        for p in NOT_PRIMES {
            assert_eq!(NaiveDivision::is_prime(*p), false);
        }
    }

    #[test]
    fn test_naive_half_division() {
        for p in PRIMES_TO_100 {
            assert_eq!(NaiveHalfDivision::is_prime(*p), true);
        }

        for p in NOT_PRIMES {
            assert_eq!(NaiveHalfDivision::is_prime(*p), false);
        }
    }
}
