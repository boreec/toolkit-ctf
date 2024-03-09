use crate::big_numbers::base10::Base10;
use std::error::Error;

#[derive(Clone, Debug)]
pub struct Base16 {
    pub value: Vec<u8>,
}

impl Base16 {
    pub fn validate_hex_string(hex_string: &str) -> Result<(), String> {
        for c in hex_string.chars() {
            if !c.is_digit(16u32) {
                return Err(format!("{} is not a correct digit in base 16", c));
            }
        }
        Ok(())
    }

    pub fn simplify_hex_string(hex_string: &str) -> String {
        let mut leading_zeros = 0;
        let mut i = 0;

        for c in hex_string.chars() {
            if c == '0' {
                leading_zeros += 1;
            } else {
                break;
            }
        }

        if leading_zeros == hex_string.len() {
            return "0".to_string();
        }
        return hex_string[leading_zeros..].to_string();
    }
}

impl PartialEq for Base16 {
    fn eq(&self, other: &Self) -> bool {
        if self.value.len() != other.value.len() {
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
        Self { value: vec![0u8] }
    }
}

impl TryFrom<&str> for Base16 {
    type Error = Box<dyn Error>;

    fn try_from(hex_string: &str) -> Result<Self, Box<dyn Error>> {
        Base16::validate_hex_string(hex_string)?;

        let padded_hex_string = if hex_string.len() % 2 == 0 {
            hex_string.to_owned()
        } else {
            format!("0{}", hex_string)
        };

        let mut bytes = Vec::with_capacity(padded_hex_string.len() / 2);

        for i in (0..padded_hex_string.len()).step_by(2) {
            let hex_pair = &padded_hex_string[i..i + 2];

            let byte = u8::from_str_radix(hex_pair, 16)?;
            bytes.push(byte);
        }

        Ok(Self { value: bytes })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_hex_string() {
        assert!(Base16::validate_hex_string("0").is_ok());
        assert!(Base16::validate_hex_string("0af21").is_ok());
        assert!(Base16::validate_hex_string("FFFFFF").is_ok());
        assert!(Base16::validate_hex_string("840983204802340823").is_ok());
        assert!(Base16::validate_hex_string("0x0000").is_err());
        assert!(Base16::validate_hex_string("dsafhodhfo").is_err());
    }

    #[test]
    fn test_base16_from_base10() {
        assert_eq!(
            Base16::from(Base10::new("0".to_string())),
            Base16 { value: vec![0u8] }
        );
    }

    #[test]
    fn test_base16_from_string() {
        assert_eq!(Vec::<u8>::new(), Base16::try_from("").unwrap().value);
        assert_eq!(vec![0u8], Base16::try_from("0").unwrap().value);
        assert_eq!(vec![10u8], Base16::try_from("A").unwrap().value);
        assert_eq!(vec![250u8], Base16::try_from("FA").unwrap().value);
        assert_eq!(vec![255u8], Base16::try_from("FF").unwrap().value);
        assert_eq!(vec![1u8, 255u8], Base16::try_from("1FF").unwrap().value);
        assert_eq!(vec![255u8, 255u8], Base16::try_from("FFFF").unwrap().value);
        assert_eq!(vec![0u8], Base16::try_from("0000").unwrap().value);
        assert!(Base16::try_from("qpwkdpq").is_err());
        assert!(Base16::try_from("x0001").is_err());
    }

    #[test]
    fn test_simplify_hex_string() {
        assert_eq!("0", Base16::simplify_hex_string("0"));
        assert_eq!("0", Base16::simplify_hex_string("00000"));
        assert_eq!("FF", Base16::simplify_hex_string("0FF"));
        assert_eq!("AF0000", Base16::simplify_hex_string("AF0000"));
        assert_eq!("12AFAFD", Base16::simplify_hex_string("12AFAFD"));
    }
}
