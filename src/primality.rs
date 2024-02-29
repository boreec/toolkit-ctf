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

    const KNOWN_PRIMES: Vec<u64> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23];

    #[test]
    fn test_naive_division() {
        assert_eq!(NaiveDivision::is_prime(2), true);
        assert_eq!(NaiveDivision::is_prime(3), true);
        assert_eq!(NaiveDivision::is_prime(5), true);
        assert_eq!(NaiveDivision::is_prime(7), true);
        assert_eq!(NaiveDivision::is_prime(11), true);
        assert_eq!(NaiveDivision::is_prime(13), true);
        assert_eq!(NaiveDivision::is_prime(17), true);
        assert_eq!(NaiveDivision::is_prime(19), true);
        assert_eq!(NaiveDivision::is_prime(23), true);

        assert_eq!(NaiveDivision::is_prime(0), false);
        assert_eq!(NaiveDivision::is_prime(1), false);
        assert_eq!(NaiveDivision::is_prime(4), false);
        assert_eq!(NaiveDivision::is_prime(6), false);
        assert_eq!(NaiveDivision::is_prime(8), false);
        assert_eq!(NaiveDivision::is_prime(9), false);
        assert_eq!(NaiveDivision::is_prime(22), false);
        assert_eq!(NaiveDivision::is_prime(14), false);
    }
}
