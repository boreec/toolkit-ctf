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

impl TryFrom<&str> for Base16 {
    type Error = &'static str;

    fn try_from(hex_string: &str) -> Result<Self, Self::Error> {
        let padded_hex_string = if hex_string.len() % 2 == 0 {
            hex_string.to_owned()
        } else {
            format!("0{}", hex_string)
        };

        let mut bytes = Vec::with_capacity(padded_hex_string.len() / 2);

        for i in (0..padded_hex_string.len()).step_by(2) {
            let hex_pair = &padded_hex_string[i..i + 2];

            if let Ok(byte) = u8::from_str_radix(hex_pair, 16) {
                bytes.push(byte);
            } else {
                return Err("Invalid hexadecimal character");
            }
        }

        Ok(Self { value: bytes })
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

    #[test]
    fn test_base16_from_string() {
        assert_eq!(Vec::<u8>::new(), Base16::try_from("").unwrap().value);
        assert_eq!(vec![0u8], Base16::try_from("0").unwrap().value);
        assert_eq!(vec![10u8], Base16::try_from("A").unwrap().value);
        assert_eq!(vec![250u8], Base16::try_from("FA").unwrap().value);
        assert_eq!(vec![255u8], Base16::try_from("FF").unwrap().value);
        assert_eq!(vec![255u8, 1u8], Base16::try_from("1FF").unwrap().value);
        assert_eq!(vec![255u8, 255u8], Base16::try_from("FFFF").unwrap().value);
    }
}
