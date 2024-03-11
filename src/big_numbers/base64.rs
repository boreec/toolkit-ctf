use std::error::Error;

const BASE_64_CHARS: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd',
    'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7',
    '8', '9', '+', '/',
];

pub struct Base64 {
    bytes: Vec<u8>,
}

impl Base64 {
    pub fn validate_base64_string(base64_string: &str) -> Result<(), String> {
        for c in base64_string.chars() {
            if !BASE_64_CHARS.contains(&c) {
                return Err(format!("{} is not a correct digit in base 64", c));
            }
        }
        Ok(())
    }
}

impl TryFrom<&str> for Base64 {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Base64::validate_base64_string(value)?;

        Ok(Self { bytes: vec![] })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_base64_string() {
        assert!(Base64::validate_base64_string("").is_ok());
        assert!(Base64::validate_base64_string(
            "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu"
        )
        .is_ok());
        assert!(Base64::validate_base64_string("???????").is_err());
    }
}
