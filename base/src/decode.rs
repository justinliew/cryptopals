use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use convert;

mod c3_tests {
    #[test]
    fn test() {
        let (ret,_,_) = super::get_best_candidate_sentence(&String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"));
        assert_eq!(ret, "Cooking MC's like a pound of bacon");
    }
}

mod c4_tests {
    #[test]
    fn test() {
        let ret = super::find_sentence();   
        assert_eq!(ret, "Now that the party is jumping\n");     
    }
}

// 65-90
// 97-122
// ETAOIN SHRDLU
fn score_candidate(candidate: &Vec<u8>) -> i32 {
    let mut common = HashMap::new();
    common.insert('e',78);
    common.insert('t',75);
    common.insert('a',72);
    common.insert('o',69);
    common.insert('i',66);
    common.insert('n',63);
    common.insert('s',60);
    common.insert('h',54);
    common.insert('r',57);
    common.insert('d',48);
    common.insert('l',51);
    common.insert('c',45);
    common.insert('u',42);

    common.insert('m',39);
    common.insert('f',36);
    common.insert('p',33);
    common.insert('g',30);
    common.insert('w',27);
    common.insert('y',24);
    common.insert('b',21);
    common.insert('v',18);
    common.insert('k',15);
    common.insert('x',12);
    common.insert('j',9);
    common.insert('q',6);
    common.insert('z',3);

    let mut score = 0;
    for i in candidate.iter() {
        let mut v = *i;
        if *i >= 65 && *i <= 90 {
            v = *i + 32;
        }

        if v >= 97 && v <= 122 {
            match common.get(&(v as char)) {
                Some(c) => score = score + c,
                None => score = score                
            }
        } else if *i < 32 || *i > 126 {
            score = score-100;
        } else {
            // this seems to break things
//            score = score+1;
        }
    }
    score
}

pub fn get_best_candidate_sentence(_input: &str) -> (String, i32, u8) {
    let mut _best_score = 0;
    let mut _best_string = String::new();
    let _input_hex = convert::hex_string_to_u8(_input);
    return get_best_candidate_sentence_from_hex_bytes(&_input_hex);
}

pub fn get_best_candidate_sentence_from_hex_bytes(input_hex: &Vec<u8>) -> (String, i32, u8) {
    let mut best_score = 0;
    let mut best_string = String::new();
    let mut best_key : u8 = 0;
    for c in 1..255 {
        let mut mask = vec![];
        for i in 0..input_hex.len() {
            mask.push(c as u8);
        }
        let candidate = convert::fixed_xor_from_u8(&input_hex, &mask);
        let score = score_candidate(&candidate);
        if score > 0 {
            println!("Score for {} is {}", c as char, score);
        }
        if score > best_score {
            best_score = score;
            best_string = convert::u8_to_string(&candidate);
            best_key = c;
        }
    }

    (best_string, best_score, best_key)
}

pub fn find_sentence()-> String {
    let mut f = File::open("4.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let lines = contents.split("\n");

    let mut best_score = 0;
    let mut best_string = String::new();
    for line in lines {
        let (out,score,_) = get_best_candidate_sentence(line);
        if score > best_score {
            best_score = score;
            best_string = out;
        }
    }
    best_string
}