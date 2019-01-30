use std::fs::File;
use std::io::prelude::*;
extern crate base;
extern crate crypto;

use crypto::aes;
use crypto::blockmodes;
use crypto::buffer;

fn main() {
// Challenge 6
//     let mut f = File::open("6.txt").expect("file not found");

//     let mut contents = String::new();
//     f.read_to_string(&mut contents)
//         .expect("something went wrong reading the file");
//    base::calc::break_vigenere_cipher_base64(&contents);
//    let encoded = String::from("JhkPTTlMBgoVBE0FHEUSERUFUA1FCxECCFAJHQQVEQEVTBENGRVNBwMXDgtBGhUACUUeDh9QGA0AWAkIHBxFAxENCE8=");
//    base::calc::break_vigenere_cipher_base64(&encoded);

// Challenge 7
    let key = String::from("YELLOW SUBMARINE");
    let mut decryptor = aes::ecb_decryptor(aes::KeySize::KeySize128, &key.as_bytes(), blockmodes::NoPadding);

    let mut f = File::open("7.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let enc = contents.as_bytes();
    let mut dec = vec![0u8; enc.len()];

    {
        let mut read_buf = buffer::RefReadBuffer::new(&enc);
        let mut write_buf = buffer::RefWriteBuffer::new(&mut dec);
        decryptor.decrypt(&mut read_buf, &mut write_buf, true);
    }

    let s = base::convert::u8_to_hex_string(&dec);
    println!("S: {}", s);
    for cur_byte in dec {
        print!("{:02x} ", cur_byte);
    }
}
