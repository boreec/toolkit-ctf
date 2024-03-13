extern crate toolkit_ctf;

use toolkit_ctf::big_numbers::base16::Base16;
use toolkit_ctf::big_numbers::remove_leading_zeros;

fn main() {
    let hex_string = "0000BADCAFE";
    println!("hex_string: {}", hex_string);

    println!(
        "is hex string valid: {:?}",
        Base16::validate_hex_string(hex_string).is_ok(),
    );

    let simplified_hex_string = remove_leading_zeros(hex_string);
    println!("simplified hex_string: {}", simplified_hex_string);
}
