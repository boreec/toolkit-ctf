extern crate toolkit_ctf;

use toolkit_ctf::big_numbers::base10::Base10;
use toolkit_ctf::big_numbers::base16::Base16;
use toolkit_ctf::utils::convert_ascii_integers_to_chars;

use base64::prelude::*;

fn enc1() {
    let nums = vec![
        99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114, 49,
        110, 116, 52, 98, 108, 51, 125,
    ];

    let chars = convert_ascii_integers_to_chars(nums);
    println!("enc1: {:?}", chars.iter().collect::<String>());
}

fn enc2() {
    let hex = "63727970746f7b596f755f77696c6c5f62655f776f726b696e675f776974685f6865785f737472696e67735f615f6c6f747d";
    let b16 = Base16::try_from(hex).unwrap();
    let chars = convert_ascii_integers_to_chars(b16.as_bytes());
    println!("enc2: {:?}", chars.iter().collect::<String>());
}

fn enc3() {
    let hex = "72bca9b68fc16ac7beeb8f849dca1d8a783e8acf9679bf9269f7bf";
    let b16 = Base16::try_from(hex).unwrap();
    let b64 = BASE64_STANDARD.encode(b16.as_bytes());
    println!("enc3: {:?}", b64);
}

fn enc4() {
    let dec = "11515195063862318899931685488813747395775516287289682636499965282714637259206269";
    let b16 = Base16::from(Base10::try_from(dec).unwrap());
    let chars = convert_ascii_integers_to_chars(b16.as_bytes());
    println!("enc4: {:?}", chars.iter().collect::<String>());
}

fn xor0() {
    let flag: String = "label".bytes().map(|x| (x ^ 13) as char).collect();
    println!("xor0: \"crypto{{{flag}}}\"");
}

fn main() {
    enc1();
    enc2();
    enc3();
    enc4();
    xor0();
}
