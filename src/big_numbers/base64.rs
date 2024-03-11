use std::error::Error;

pub struct Base64 {
    bytes: Vec<u8>,
}

impl Base64 {
    pub fn validate_base64_string(base64_string: &str) -> Result<(), String> {
        Ok(())
    }
}
// impl TryFrom<&str> for Base64 {
//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         Ok(())
//     }
// }
