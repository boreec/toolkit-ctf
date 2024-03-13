pub mod base10;
pub mod base16;
pub mod base64;

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
