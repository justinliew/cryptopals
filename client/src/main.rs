use std::fs::File;
use std::io::prelude::*;
extern crate base;

fn main() {
    // let mut f = File::open("6.txt").expect("file not found");

    // let mut contents = String::new();
    // f.read_to_string(&mut contents)
    //     .expect("something went wrong reading the file");
//    base::calc::break_vigenere_cipher_base64(&contents);

    // let contents = String::from("fuse fuel for falling flocks");
    // let key = String::from("few");

    // let encoded = base::calc::repeating_xor_from_bytes(contents.as_bytes(), key.as_bytes());
    // let encoded_hex = base::calc::hex_string_to_base64_string(&base::convert::u8_to_hex_string(&encoded));
    // println!("Encoded: {}", encoded_hex);

    let encoded = String::from("JhkPTTlMBgoVBE0FHEUSERUFUA1FCxECCFAJHQQVEQEVTBENGRVNBwMXDgtBGhUACUUeDh9QGA0AWAkIHBxFAxENCE8=");
    base::calc::break_vigenere_cipher_base64(&encoded);
}
