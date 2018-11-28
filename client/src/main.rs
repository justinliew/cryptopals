use std::fs::File;
use std::io::prelude::*;
extern crate base;

fn main() {
    // let mut _f = File::open("6.txt").expect("file not found");

    // let mut _contents = String::new();
    // _f.read_to_string(&mut _contents)
    //     .expect("something went wrong reading the file");

    

    base::calc::break_vigenere_cipher(&_contents);
}
