use crate::big_numbers::base10::Base10;

#[derive(Debug)]
struct Base16 {
    pub value: Vec<u8>,
}

impl PartialEq for Base16 {
    fn eq(&self, other: &Self) -> bool {
        if self.value.len() == other.value.len() {
            return false;
        }

        let mut i = 0;
        while i < self.value.len() && self.value[i] == other.value[i] {
            i += 1;
        }

        i >= self.value.len()
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
