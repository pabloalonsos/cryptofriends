extern crate rustc_serialize as serialize;
use self::serialize::base64::{ToBase64, STANDARD};
use self::serialize::hex::{FromHex, ToHex};

fn hex_to_base64(hex_input: &str) -> String {
    hex_input
        .from_hex()
        .unwrap()
        .to_base64(STANDARD)
}

fn fixed_xor(xor_input: &str, xored_against: &str) -> String {
    let xor_input_unhexed: Vec<u8> = xor_input
        .from_hex()
        .unwrap();

    let xored_against_unhexed: Vec<u8> = xored_against
        .from_hex()
        .unwrap();

    if xor_input_unhexed.len() != xored_against_unhexed.len() {
        panic!("This is how betrayal feels like, huh?");
    }

    let xored_result: Vec<u8> = xor_input_unhexed
        .iter()
        .zip(xored_against_unhexed.iter())
        .map(|(&x, &y)| x ^ y)
        .collect();

    xored_result.to_hex()
}

fn main() {
    println!("Challenge 1: Convert hex to base64");
    let hex_input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let base64_output = hex_to_base64(hex_input);
    println!("Result: {}", base64_output);

    println!("Challenge 2: Fixed XOR");
    let xor_input = "1c0111001f010100061a024b53535009181c";
    let xored_against = "686974207468652062756c6c277320657965";
    let xored_output = fixed_xor(xor_input, xored_against);
    println!("Result: {}", xored_output);
}
