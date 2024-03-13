pub mod base10;
pub mod base16;
pub mod base64;

/// Removes leading zeros from a string representing a number regardless of its
/// base.
/// "0101010101" -> "101010101"
/// "00FF" -> "FF"
pub fn remove_leading_zeros(number_string: &str) -> String {
    let mut leading_zeros = 0;

    for c in number_string.chars() {
        if c == '0' {
            leading_zeros += 1;
        } else {
            break;
        }
    }

    if leading_zeros == number_string.len() {
        return "0".to_string();
    }
    return number_string[leading_zeros..].to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_leading_zeros() {
        assert_eq!("0", remove_leading_zeros("0"));
        assert_eq!("0", remove_leading_zeros("00000"));
        assert_eq!("10000", remove_leading_zeros("10000"));
        assert_eq!("10001", remove_leading_zeros("0010001"));
        assert_eq!("BADCAFE", remove_leading_zeros("00BADCAFE"));
    }
}
