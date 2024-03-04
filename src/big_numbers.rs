mod big_numbers {
    pub struct Base10 {
        value: String,
    }

    impl Base10 {
        pub fn new(value: String) -> Self {
            Self { value }
        }
    }

    pub struct Base16 {
        value: String,
    }

    impl From<Base10> for Base16 {
        fn from(decimal_number: Base10) -> Self {
            Self {
                value: "".to_string(),
            }
        }
    }
}
