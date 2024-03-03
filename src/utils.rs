pub fn convert_ascii_integers_to_chars(ascii_integers: Vec<u8>) -> Vec<char> {
    ascii_integers.iter().map(|&i| i as char).collect()
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
}
