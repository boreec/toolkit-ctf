#[derive(Debug)]
pub struct Base10 {
    value: String,
}

impl Base10 {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn validate_decimal_string(decimal_string: &str) -> Result<(), String> {
        for c in decimal_string.chars() {
            if !c.is_digit(10u32) {
                return Err(format!("{} is not a correct digit in base 16", c));
            }
        }
        Ok(())
    }
}

impl PartialEq for Base10 {
    fn eq(&self, other: &Self) -> bool {
        if self.value.len() != other.value.len() {
            return false;
        }

        for i in 0..self.value.len() {
            if self.value[i] != other.value[i] {
                return false;
            }
        }

        return true;
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
