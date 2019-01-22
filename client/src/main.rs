use std::fs::File;
use std::io::prelude::*;
extern crate base;

fn main() {
    // let mut f = File::open("6.txt").expect("file not found");

    // let mut contents = String::new();
    // f.read_to_string(&mut contents)
    //     .expect("something went wrong reading the file");

    let contents = String::from("fuse fuel for falling flocks");
    base::calc::break_vigenere_cipher(&contents.as_bytes().to_vec());

//    base::calc::break_vigenere_cipher_base64(&contents.as_bytes());
}
