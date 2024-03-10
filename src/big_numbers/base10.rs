#[derive(Debug)]
pub struct Base10 {
    pub bytes: Vec<u8>,
}

impl Base10 {
    pub fn validate_decimal_string(decimal_string: &str) -> Result<(), String> {
        for c in decimal_string.chars() {
            if !c.is_digit(10u32) {
                return Err(format!("{} is not a correct digit in base 16", c));
            }
        }
        Ok(())
    }

    pub fn simplify_decimal_string(decimal_string: &str) -> String {
        let mut leading_zeros = 0;

        for c in decimal_string.chars() {
            if c == '0' {
                leading_zeros += 1;
            } else {
                break;
            }
        }

        if leading_zeros == decimal_string.len() {
            return "0".to_string();
        }
        return decimal_string[leading_zeros..].to_string();
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

impl TryFrom<&str> for Base16 {
    type Error = Box<dyn Error>;

    fn try_from(decimal_string: &str) -> Result<Self, Box<dyn Error>> {
        if decimal_string.is_empty() {
            return Ok(Self { bytes: Vec::new() });
        }
        Base10::validate_hex_string(decimal_string)?;
        let simplified_hex_string = Base10::simplify_hex_string(hex_string);

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
    fn test_validate_decimal_string() {
        assert!(Base10::validate_decimal_string("0").is_ok());
        assert!(Base10::validate_decimal_string("00000").is_ok());
        assert!(Base10::validate_decimal_string("1").is_ok());
        assert!(Base10::validate_decimal_string("840983204802340823").is_ok());
        assert!(Base10::validate_decimal_string("0x0000").is_err());
        assert!(Base10::validate_decimal_string("dsafhodhfo").is_err());
        assert!(Base10::validate_decimal_string("BADCAFE").is_err());
    }
}
