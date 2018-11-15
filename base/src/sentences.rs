use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

mod c3_tests {
    #[test]
    fn test() {
        let (_ret,_i) = super::get_best_candidate_sentence(&String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"));
        assert_eq!(_ret, "Cooking MC's like a pound of bacon");
    }
}

mod c4_tests {
    #[test]
    fn test() {
        super::find_sentence();        
    }
}

// 65-90
// 97-122
// ETAOIN SHRDLU
fn score_candidate(_candidate: &Vec<u8>) -> i32 {
    let mut common = HashMap::new();
    common.insert('e',3);
    common.insert('t',3);
    common.insert('a',3);
    common.insert('o',3);
    common.insert('i',2);
    common.insert('n',2);
    common.insert('s',2);
    common.insert('h',2);
    common.insert('r',2);
    common.insert('d',2);
    common.insert('l',2);
    common.insert('u',2);

    let mut _score = 0;
    for _i in _candidate.iter() {
        let mut _v = *_i;
        if *_i >= 65 && *_i <= 90 {
            _v = *_i + 32;
        }

        if _v >= 97 && _v <= 122 {
            match common.get(&(_v as char)) {
                Some(_c) => _score = _score + _c,
                None => _score = _score + 1                
            }
        } else if *_i < 32 || *_i > 126 {
            _score = _score-1;
        } else {
            // this seems to break things
//            _score = _score+1;
        }
    }
    _score
}

pub fn get_best_candidate_sentence(_input: &str) -> (String, i32) {
    let mut _best_score = 0;
    let mut _best_string = String::new();
    for _c in 1..256 {
        let mut _mask = vec![];
        for _i in 0.._input.len() {
            _mask.push(_c as u8);
        }
        let _input_hex = input_helpers::hex_string_to_u8(_input);
        let _candidate = fixed_xor_from_u8(&_input_hex, &_mask);
        let _score = score_candidate(&_candidate);
        if _score > _best_score {
            _best_score = _score;
            _best_string = input_helpers::u8_to_string(&_candidate);
        }
    }

    (_best_string, _best_score)
}

pub fn find_sentence() {
    let mut _f = File::open("4.txt").expect("file not found");

    let mut _contents = String::new();
    _f.read_to_string(&mut _contents)
        .expect("something went wrong reading the file");
    let _lines = _contents.split("\n");

    let mut _best_score = 0;
    let mut _best_string = String::new();
    for _line in _lines {
        let (_out,_score) = get_best_candidate_sentence(_line);
        if _score > _best_score {
            _best_score = _score;
            _best_string = _out;
        }
    }
    println!("{} -> {}", _best_string, _best_score);
}