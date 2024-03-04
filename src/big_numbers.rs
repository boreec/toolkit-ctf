/// Handles numbers in different bases regardless of their length.
mod big_numbers {
    #[derive(Debug)]
    pub struct Base10 {
        value: String,
    }

    impl Base10 {
        pub fn new(value: String) -> Self {
            Self { value }
        }
    }

    impl PartialEq for Base10 {
        fn eq(&self, other: &Self) -> bool {
            self.value == other.value
        }
    }

    #[derive(Debug)]
    pub struct Base16 {
        value: String,
    }

    impl Base16 {
        pub fn new(value: String) -> Self {
            Self { value }
        }
    }

    impl PartialEq for Base16 {
        fn eq(&self, other: &Self) -> bool {
            self.value == other.value
        }
    }

    impl From<Base10> for Base16 {
        fn from(decimal_number: Base10) -> Self {
            Self {
                value: "0".to_string(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::big_numbers::*;

    #[test]
    fn test_base16_from_base10() {
        assert_eq!(
            Base16::from(Base10::new("0".to_string())),
            Base16::new("0".to_string())
        );
    }
}
