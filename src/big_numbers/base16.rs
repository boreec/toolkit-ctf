use crate::big_numbers::base10::Base10;

#[derive(Debug)]
struct Base16 {
    pub value: Vec<u8>,
}

impl PartialEq for Base16 {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl From<Base10> for Base16 {
    fn from(decimal_number: Base10) -> Self {
        Self { value: vec![0] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base16_from_base10() {
        assert_eq!(
            Base16::from(Base10::new("0".to_string())),
            Base16 { value: vec![0] }
        );
    }
}
