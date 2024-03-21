use std::error::Error;

use super::remove_leading_zeros;

#[derive(Debug)]
pub struct Base10 {
    pub bytes: Vec<u8>,
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
        if self.bytes.len() != other.bytes.len() {
            return false;
        }

        for i in 0..self.bytes.len() {
            if self.bytes[i] != other.bytes[i] {
                return false;
            }
        }

        return true;
    }
}

impl TryFrom<&str> for Base10 {
    type Error = Box<dyn Error>;

    fn try_from(decimal_string: &str) -> Result<Self, Box<dyn Error>> {
        if decimal_string.is_empty() {
            return Ok(Self { bytes: Vec::new() });
        }
        Base10::validate_decimal_string(decimal_string)?;
        let mut simplified_string = remove_leading_zeros(decimal_string);

        // rough estimate
        let bytes_capacity = simplified_string.len() as f64 * 0.3;
        let mut bytes = Vec::with_capacity(bytes_capacity as usize);

        while !simplified_string.is_empty() {
            let slice_size = 3.min(simplified_string.len());
            let slice_chars =
                &simplified_string[simplified_string.len() - slice_size..];
            let slice_value: u16 = slice_chars.parse()?;
            if slice_value > 255 {
                bytes.push(255u8);
                let remainder = slice_value - 255;
                simplified_string
                    .truncate(simplified_string.len() - slice_size);
                simplified_string.push_str(&remainder.to_string());
            } else {
                bytes.push(slice_value as u8);
                simplified_string
                    .truncate(simplified_string.len() - slice_size);
            }
        }

        Ok(Self { bytes })
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
        assert_eq!(vec![0u8], Base10::try_from("0").unwrap().bytes);
        assert_eq!(vec![255u8], Base10::try_from("255").unwrap().bytes);
        assert_eq!(vec![255u8, 1u8], Base10::try_from("256").unwrap().bytes);
    }
}
