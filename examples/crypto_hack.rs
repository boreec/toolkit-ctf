extern crate toolkit_ctf;

use toolkit_ctf::big_numbers::base10::Base10;
use toolkit_ctf::big_numbers::base16::{Base16, XorStrategy};
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
    let chars = convert_ascii_integers_to_chars(b16.as_le_bytes());
    println!("enc2: {:?}", chars.iter().collect::<String>());
}

fn enc3() {
    let hex = "72bca9b68fc16ac7beeb8f849dca1d8a783e8acf9679bf9269f7bf";
    let b16 = Base16::try_from(hex).unwrap();
    let b64 = BASE64_STANDARD.encode(b16.as_le_bytes());
    println!("enc3: {:?}", b64);
}

fn enc4() {
    let dec = "11515195063862318899931685488813747395775516287289682636499965282714637259206269";
    let b16 = Base16::from(Base10::try_from(dec).unwrap());
    let chars = convert_ascii_integers_to_chars(b16.as_le_bytes());
    println!("enc4: {:?}", chars.iter().collect::<String>());
}

fn xor0() {
    let flag: String = "label".bytes().map(|x| (x ^ 13) as char).collect();
    println!("xor0: \"crypto{{{flag}}}\"");
}

fn xor1() {
    let flag = Base16::xor_numbers(vec![
        Base16::try_from(
            "04ee9855208a2cd59091d04767ae47963170d1660df7f56f5faf",
        )
        .unwrap(),
        Base16::try_from(
            "c1545756687e7573db23aa1c3452a098b71a7fbf0fddddde5fc1",
        )
        .unwrap(),
        Base16::try_from(
            "a6c8b6733c9b22de7bc0253266a3867df55acde8635e19c73313",
        )
        .unwrap(),
    ])
    .unwrap();

    let chars = convert_ascii_integers_to_chars(flag.as_le_bytes());
    println!("xor1: {:?}", chars.iter().collect::<String>());
}

fn xorkey0() {
    let hex_string =
        "73626960647f6b206821204f21254f7d694f7624662065622127234f726927756d";
    let b16 = Base16::try_from(hex_string).unwrap();
    let mut decoded = String::new();
    let mut i = 0u8;
    while i != 255u8 && !decoded.contains("crypto{") {
        let xor = b16.xor(
            &Base16 {
                be_bytes: vec![i; hex_string.len() / 2],
            },
            &XorStrategy::Repeating,
        );
        let chars = convert_ascii_integers_to_chars(xor.as_le_bytes());
        decoded = chars.iter().collect();
        if decoded.contains("crypto{") {
            break;
        }
        i += 1;
    }
    println!("xorkey0: {:?}", decoded);
}

fn xorkey1() {
    let secret = "0e0b213f26041e480b26217f27342e175d0e070a3c5b103e2526217f27342e175d0e077e263451150104";
    let secret_b16 = Base16::try_from(secret).unwrap();
    let secret_bytes = secret_b16.as_le_bytes();
    let flag_bytes = b"crypto{".to_vec();
    let xor_partial_key = Base16::from_le_bytes(flag_bytes.clone())
        .xor(&secret_b16, &XorStrategy::Repeating);
    let partial_key =
        convert_ascii_integers_to_chars(xor_partial_key.as_le_bytes());
    let partial_key_str =
        partial_key.iter().collect::<String>()[..flag_bytes.len()].to_string()
            + "y";
    let mut complete_key = String::new();
    let mut i = 0;
    while i < secret_bytes.len() {
        complete_key += partial_key_str
            .chars()
            .nth(i % partial_key_str.len())
            .unwrap()
            .to_string()
            .as_str();
        i += 1;
    }

    let complete_key_b16 = Base16::from_le_bytes(complete_key.into_bytes());
    let xor = complete_key_b16.xor(&secret_b16, &XorStrategy::Repeating);
    let chars = convert_ascii_integers_to_chars(xor.as_le_bytes());
    println!("xorkey1: {:?}", chars.iter().collect::<String>());
}

fn main() {
    enc1();
    enc2();
    enc3();
    enc4();
    xor0();
    xor1();
    xorkey0();
    xorkey1();
}
