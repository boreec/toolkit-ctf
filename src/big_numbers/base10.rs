use std::error::Error;

use super::remove_leading_zeros;

/// Represents a number in base 10.
#[derive(Debug)]
pub struct Base10 {
    /// bytes stored in a big endian order
    pub be_bytes: Vec<u8>,
}

impl Base10 {
    pub fn validate_decimal_string(decimal_string: &str) -> Result<(), String> {
        for c in decimal_string.chars() {
            if !c.is_digit(10u32) {
                return Err(format!("{} is not a correct digit in base 10", c));
            }
        }
        Ok(())
    }
}

impl PartialEq for Base10 {
    fn eq(&self, other: &Self) -> bool {
        if self.be_bytes.len() != other.be_bytes.len() {
            return false;
        }

        for i in 0..self.be_bytes.len() {
            if self.be_bytes[i] != other.be_bytes[i] {
                return false;
            }
        }

        true
    }
}

impl TryFrom<&str> for Base10 {
    type Error = Box<dyn Error>;

    fn try_from(decimal_string: &str) -> Result<Self, Box<dyn Error>> {
        if decimal_string.is_empty() {
            return Ok(Self {
                be_bytes: Vec::new(),
            });
        }

        if decimal_string == "0" {
            return Ok(Self {
                be_bytes: vec![0u8],
            });
        }

        Base10::validate_decimal_string(decimal_string)?;
        let simplified_string = remove_leading_zeros(decimal_string);

        let num: num_bigint::BigInt = simplified_string.parse().unwrap();

        let mut bytes: Vec<u8> = num
            .iter_u32_digits()
            .into_iter()
            .map(|v| v.to_le_bytes().into_iter())
            .flatten()
            .rev()
            .skip_while(|v| *v == 0)
            .collect();

        bytes.reverse();

        Ok(Self { be_bytes: bytes })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_decimal_string() {
        assert!(Base10::validate_decimal_string("0").is_ok());
        assert!(Base10::validate_decimal_string("00000").is_ok());
        assert!(Base10::validate_decimal_string("1").is_ok());
        assert!(Base10::validate_decimal_string("840983204802340823").is_ok());
        assert!(Base10::validate_decimal_string("0x0000").is_err());
        assert!(Base10::validate_decimal_string("dsafhodhfo").is_err());
        assert!(Base10::validate_decimal_string("BADCAFE").is_err());
    }

    #[test]
    fn test_try_from_string() {
        assert!(Base10::try_from("0").is_ok());
        assert!(Base10::try_from("adjsakjdlas").is_err());
        assert_eq!(vec![0u8], Base10::try_from("0").unwrap().be_bytes);
        assert_eq!(vec![255u8], Base10::try_from("255").unwrap().be_bytes);
        assert_eq!(vec![0u8, 1u8], Base10::try_from("256").unwrap().be_bytes);
        assert_eq!(vec![1u8, 1u8], Base10::try_from("257").unwrap().be_bytes);
        assert_eq!(vec![0u8, 2u8], Base10::try_from("512").unwrap().be_bytes);
        assert_eq!(vec![1u8, 2u8], Base10::try_from("513").unwrap().be_bytes);
        assert_eq!(vec![9u8, 3u8], Base10::try_from("777").unwrap().be_bytes);
        assert_eq!(
            vec![0u8, 0u8, 1u8],
            Base10::try_from("65536").unwrap().be_bytes
        );
    }
}
