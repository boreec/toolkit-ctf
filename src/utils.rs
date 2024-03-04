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

pub fn decode_hex_to_bytes(hex_string: &str) -> Result<Vec<u8>, &'static str> {
    let result: Vec<u8> = vec![];
    let padded_hex_string = if hex_string.len() % 2 == 0 {
        hex_string.to_owned()
    } else {
        format!("0{}", hex_string)
    };

    let mut bytes = Vec::with_capacity(padded_hex_string.len() / 2);

    for i in (0..padded_hex_string.len()).step_by(2) {
        let hex_pair = &padded_hex_string[i..i + 2];

        if let Ok(byte) = u8::from_str_radix(hex_pair, 16) {
            bytes.push(byte);
        } else {
            return Err("Invalid hexadecimal character");
        }
    }

    Ok(bytes)
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
    fn test_decode_hex_to_bytes() {
        assert_eq!(Ok(vec![]), decode_hex_to_bytes(""));
        assert_eq!(Ok(vec![0]), decode_hex_to_bytes("0"));
        assert_eq!(Ok(vec![9]), decode_hex_to_bytes("9"));
        assert_eq!(Ok(vec![10]), decode_hex_to_bytes("A"));
        assert_eq!(Ok(vec![15]), decode_hex_to_bytes("F"));

        let bytes = decode_hex_to_bytes("63727970746f7b596f755f77696c6c5f62655f776f726b696e675f776974685f6865785f737472696e67735f615f6c6f747d").unwrap();
        let chars = convert_ascii_integers_to_chars(bytes);
        let encoded_msg: String = chars.into_iter().collect();
        assert_eq!(
            encoded_msg,
            "crypto{You_will_be_working_with_hex_strings_a_lot}",
        );
    }

    #[test]
    fn test_encode_bytes_to_base64() {
        let bytes = decode_hex_to_bytes(
            "72bca9b68fc16ac7beeb8f849dca1d8a783e8acf9679bf9269f7bf",
        )
        .unwrap();
        assert_eq!(
            encode_bytes_to_base64(bytes.clone()),
            "crypto/Base+64+Encoding+is+Web+Safe/",
        );
    }

    #[test]
    fn test_base10_string_to_hex_string() {
        let base10_str = "310400273487";
        assert_eq!("48454C4C4F", base10_string_to_hex_string(base10_str));

        let base10_str = "11515195063862318899931685488813747395775516287289682636499965282714637259206269";
        let hex_str = base10_string_to_hex_string(base10_str);
        let bytes = decode_hex_to_bytes(&hex_str).unwrap();
        let chars = convert_ascii_integers_to_chars(bytes);
        let encoded_message: String = chars.into_iter().collect();
        assert!(encoded_message == "foo", "encoded_msg: {}", encoded_message);
    }
}
