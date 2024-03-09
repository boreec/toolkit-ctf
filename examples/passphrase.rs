extern crate toolkit_ctf;

use toolkit_ctf::generators::passphrase::*;

fn main() {
    println!("loading words frequency list...");
    let frequency_list = load_frequency_list().unwrap();

    println!(
        "passphrase with min_length 30: {}",
        generate_passphrase(30, &frequency_list).unwrap(),
    );
}
