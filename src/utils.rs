pub fn convert_ascii_integers_to_chars(ascii_integers: Vec<u8>) -> Vec<char> {
    ascii_integers.iter().map(|&i| i as char).collect()
}

pub fn base10_string_to_hex_string(base10_string: &str) -> String {
    if let Ok(decimal) = base10_string.parse::<u128>() {
        format!("{:X}", decimal)
    } else {
        String::from("Invalid input: Not a valid decimal number")
    }
}

pub fn encode_bytes_to_base64(bytes: Vec<u8>) -> String {
    let mut result = String::new();
    let base64_chars: Vec<char> =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
            .chars()
            .collect();

    for chunk in bytes.chunks(3) {
        let mut num = 0u32;

        for (i, &byte) in chunk.iter().enumerate() {
            num |= (byte as u32) << (16 - i * 8);
        }

        for i in 0..4 {
            let index = ((num >> (18 - i * 6)) & 0b0011_1111) as usize;
            result.push(base64_chars[index]);
        }

        // Add padding '=' if needed
        if chunk.len() < 3 {
            let padding_count = 3 - chunk.len();
            result.push_str(&"=".repeat(padding_count));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::big_numbers::base16::Base16;

    #[test]
    fn test_convert_ascii_integers_to_chars() {
        let ascii_integers: Vec<u8> = vec![
            99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114,
            49, 110, 116, 52, 98, 108, 51, 125,
        ];

        let expected: Vec<char> = vec![
            'c', 'r', 'y', 'p', 't', 'o', '{', 'A', 'S', 'C', 'I', 'I', '_',
            'p', 'r', '1', 'n', 't', '4', 'b', 'l', '3', '}',
        ];

        assert_eq!(expected, convert_ascii_integers_to_chars(ascii_integers));
    }

    #[test]
    fn test_encode_bytes_to_base64() {
        let bytes = Base16::try_from(
            "72bca9b68fc16ac7beeb8f849dca1d8a783e8acf9679bf9269f7bf",
        );
        assert!(bytes.is_ok());

        assert_eq!(
            encode_bytes_to_base64(bytes.unwrap().value),
            "crypto/Base+64+Encoding+is+Web+Safe/",
        );
    }
}
