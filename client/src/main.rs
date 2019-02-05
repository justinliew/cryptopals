use std::fs::File;
use std::io::prelude::*;
extern crate base;
extern crate crypto;

use std::collections::HashSet;
use std::fmt::{self, Formatter, Display};

//use base::block;

// use crypto;
// use crypto::aes;
// use crypto::blockmodes;
// use crypto::buffer;

fn c6() {
    let mut f = File::open("6.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
   base::calc::break_vigenere_cipher_base64(&contents);
   let encoded = String::from("JhkPTTlMBgoVBE0FHEUSERUFUA1FCxECCFAJHQQVEQEVTBENGRVNBwMXDgtBGhUACUUeDh9QGA0AWAkIHBxFAxENCE8=");
   base::calc::break_vigenere_cipher_base64(&encoded);
}

fn c7() {
    // let key = String::from("YELLOW SUBMARINE");
    // let mut decryptor = aes::ecb_decryptor(aes::KeySize::KeySize128, &key.as_bytes(), blockmodes::NoPadding);

    // let mut f = File::open("7.txt").expect("file not found");

    // let mut contents = String::new();
    // f.read_to_string(&mut contents)
    //     .expect("something went wrong reading the file");

    // let enc = base::calc::decode_base64_to_bytes(&contents);
    // let mut dec = vec![0u8; enc.len()];

    // {
    //     let mut read_buf = buffer::RefReadBuffer::new(&enc);
    //     let mut write_buf = buffer::RefWriteBuffer::new(&mut dec);
    //     decryptor.decrypt(&mut read_buf, &mut write_buf, true);
    // }

    // let s = base::convert::u8_to_string(&dec);
    // println!("{}", s);
}

fn c8() {
    // let mut f = File::open("8.txt").expect("file not found");

    // let mut contents = String::new();
    // f.read_to_string(&mut contents)
    //     .expect("something went wrong reading the file");
    // let lines = contents.split("\n");

    // type LineCalc = (i32,usize);
    // let mut line_calcs : Vec<LineCalc> = Vec::new();

    // let mut idx = 0;
    // for line in lines {
    //     if line.len() == 0 {
    //         continue
    //     }
    //     let mut set = HashSet::new();
    //     let len = line.len();

    //     for b in 0..(len/16) {
    //         let s = &line[b*16..b*16+16];
    //         if idx == 132 {
    //             println!("{}", s);
    //         }
    //         set.insert(s);
    //     }
    //     line_calcs.push((idx, len/16 - set.len()));
    //     idx = idx + 1;
    // }
    // line_calcs.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());
    // println!("{} of {}", line_calcs[line_calcs.len()-1].0, idx+1);
}

fn c10() {
    // let mut f = File::open("10.txt").expect("file not found");

    // let mut contents = String::new();
    // f.read_to_string(&mut contents)
    //     .expect("something went wrong reading the file");
    // let stripped : String = contents.chars().filter(|&c| c != '\n').collect();
    // let decrypted = block::cbc::decrypt("YELLOW SUBMARINE", &stripped, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    // println!("Decrypted: {}", decrypted);
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "RGB ({},{},{}) 0x{:X}{:X}{:X}", self.red, self.green, self.blue, self.red, self.green, self.blue)
    }
}

fn main() {
//    c8();
//    c10();

    let c : Color = Color{red:128,green:255,blue:90};
    println!("{}", c);

}
