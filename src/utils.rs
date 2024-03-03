pub fn convert_ascii_integers_to_chars(ascii_integers: Vec<u8>) -> Vec<char> {
    let mut chars: Vec<char> = vec![];
    for i in ascii_integers.iter() {
        chars.push(i.clone() as char);
    }
    return chars;
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
}
