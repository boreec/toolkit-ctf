use base64::prelude::*;
use std::error::Error;

/// Represents all available chars for base 64 numbers.
const BASE_64_CHARS: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd',
    'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7',
    '8', '9', '+', '/',
];

/// Represents the character used for padding base 64 numbers.
const BASE_64_PAD: char = '=';

/// Represents a base 64 number of any length.
pub struct Base64 {
    bytes: Vec<u8>,
}

impl Base64 {
    pub fn validate_base64_string(base64_string: &str) -> Result<(), String> {
        if base64_string.len() % 4 != 0 {
            return Err(
                "base64 string length must be divisible by 4".to_string()
            );
        }
        for c in base64_string.chars() {
            if !BASE_64_CHARS.contains(&c) && c != BASE_64_PAD {
                return Err(format!("{} is not a correct digit in base 64", c));
            }
        }
        Ok(())
    }
}

impl PartialEq for Base64 {
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

impl TryFrom<&str> for Base64 {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Base64::validate_base64_string(value)?;

        Ok(Self {
            bytes: BASE64_STANDARD.decode(value)?,
        })
    }
}

impl Into<String> for Base64 {
    fn into(self) -> String {
        let mut result = String::new();

        for chunk in self.bytes.chunks(3) {
            let mut num = 0u32;

            for (i, &byte) in chunk.iter().enumerate() {
                num |= (byte as u32) << (16 - i * 8);
            }

            for i in 0..4 {
                let index = ((num >> (18 - i * 6)) & 0b0011_1111) as usize;
                result.push(BASE_64_CHARS[index]);
            }

            // Add padding '=' if needed
            if chunk.len() < 3 {
                let padding_count = 3 - chunk.len();
                result.push_str(&"=".repeat(padding_count));
            }
        }

        result
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

    #[test]
    fn test_try_from_string() {
        assert_eq!(Vec::<u8>::new(), Base64::try_from("").unwrap().bytes);
        assert_eq!(vec![65u8], Base64::try_from("QQ==").unwrap().bytes);
        assert_eq!(vec![90u8], Base64::try_from("Wg==").unwrap().bytes);
        assert_eq!(vec![65u8, 66u8], Base64::try_from("QUI=").unwrap().bytes);
        assert_eq!(
            vec![65u8, 66u8, 67u8],
            Base64::try_from("QUJD").unwrap().bytes
        );
    }
}
