use std::fs::File;
use std::io::prelude::*;
extern crate base;

fn main() {
    let mut f = File::open("6.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
   base::calc::break_vigenere_cipher_base64(&contents);

    // let encoded = String::from("JhkPTTlMBgoVBE0FHEUSERUFUA1FCxECCFAJHQQVEQEVTBENGRVNBwMXDgtBGhUACUUeDh9QGA0AWAkIHBxFAxENCE8=");
    // base::calc::break_vigenere_cipher_base64(&encoded);
}
