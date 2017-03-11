extern crate rustc_serialize as serialize;
use self::serialize::base64::{ToBase64, STANDARD};
use self::serialize::hex::FromHex;

fn hex_to_base64(hex_input: &str) -> String {
    hex_input
        .from_hex()
        .unwrap()
        .to_base64(STANDARD)
}

fn main() {
    println!("Challenge 1: Convert hex to base64");
    let hex_input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let base64_output = hex_to_base64(hex_input);
    println!("Result: {}", base64_output);
}
