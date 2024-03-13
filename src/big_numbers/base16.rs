use crate::big_numbers::base10::Base10;
use std::error::Error;

use super::remove_leading_zeros;

#[derive(Clone, Debug)]
pub struct Base16 {
    pub bytes: Vec<u8>,
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
}

impl PartialEq for Base16 {
    fn eq(&self, other: &Self) -> bool {
        if self.bytes.len() != other.bytes.len() {
            return false;
        }

        let mut i = 0;
        while i < self.bytes.len() && self.bytes[i] == other.bytes[i] {
            i += 1;
        }

        i >= self.bytes.len()
    }
}

impl From<Base10> for Base16 {
    fn from(decimal_number: Base10) -> Self {
        Self {
            bytes: decimal_number.bytes,
        }
    }
}

impl TryFrom<&str> for Base16 {
    type Error = Box<dyn Error>;

    fn try_from(hex_string: &str) -> Result<Self, Box<dyn Error>> {
        if hex_string.is_empty() {
            return Ok(Self { bytes: Vec::new() });
        }
        Base16::validate_hex_string(hex_string)?;
        let simplified_hex_string = remove_leading_zeros(hex_string);

        let padded_hex_string = if simplified_hex_string.len() % 2 == 0 {
            simplified_hex_string.to_owned()
        } else {
            format!("0{}", simplified_hex_string)
        };

        let mut bytes = Vec::with_capacity(padded_hex_string.len() / 2);

        for i in (0..padded_hex_string.len()).step_by(2) {
            let hex_pair = &padded_hex_string[i..i + 2];

            let byte = u8::from_str_radix(hex_pair, 16)?;
            bytes.push(byte);
        }

        bytes.reverse();
        Ok(Self { bytes })
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
            Base16::from(Base10::try_from("720640").unwrap()),
            Base16 {
                bytes: vec![0u8, 255u8, 10u8]
            }
        );
    }

    #[test]
    fn test_base16_from_string() {
        assert_eq!(Vec::<u8>::new(), Base16::try_from("").unwrap().bytes);
        assert_eq!(vec![0u8], Base16::try_from("0").unwrap().bytes);
        assert_eq!(vec![10u8], Base16::try_from("A").unwrap().bytes);
        assert_eq!(vec![250u8], Base16::try_from("FA").unwrap().bytes);
        assert_eq!(vec![255u8], Base16::try_from("FF").unwrap().bytes);
        assert_eq!(vec![255u8, 1u8], Base16::try_from("1FF").unwrap().bytes);
        assert_eq!(vec![255u8, 160u8], Base16::try_from("A0FF").unwrap().bytes);
        assert_eq!(vec![255u8, 255u8], Base16::try_from("FFFF").unwrap().bytes);
        assert_eq!(vec![0u8], Base16::try_from("0000").unwrap().bytes);
        assert!(Base16::try_from("qpwkdpq").is_err());
        assert!(Base16::try_from("x0001").is_err());
    }
}
