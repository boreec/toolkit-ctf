pub struct Base64 {
    bytes: Vec<u8>,
}

impl Base64 {
    pub fn validate_base64_string(base64_string: &str) -> Result<(), String> {
        let valid_chars: Vec<char> =
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
                .chars()
                .collect();

        for c in base64_string.chars() {
            if !valid_chars.contains(&c) {
                return Err(format!("{} is not a correct digit in base 64", c));
            }
        }
        Ok(())
    }
}
// impl TryFrom<&str> for Base64 {
//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         Ok(())
//     }
// }

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
