#[cfg(test)]
mod c1_tests {
    #[test]
    fn example() {
        let output = super::hex_to_base64(&String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"));
        assert_eq!(output, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }

    #[test]
    fn single() {
        let output = super::hex_to_base64(&String::from("1"));
        assert_eq!(output, "AQ==");
    }

    #[test]
    fn double() {
        let output = super::hex_to_base64(&String::from("12"));
        assert_eq!(output, "Eg==");
    }

    #[test]
    fn simplest_example() {
        let output = super::hex_to_base64(&String::from("010101"));
        assert_eq!(output, "AQEB");
    }
}

#[cfg(test)]
mod c2_tests {
    #[test]
    fn example() {
        let output = super::fixed_xor(&String::from("1c0111001f010100061a024b53535009181c"), &String::from("686974207468652062756c6c277320657965"));
        assert_eq!(output, "746865206b696420646f6e277420706c6179");
    }

    #[test]
    fn start_of_example() {
        let output = super::fixed_xor(&String::from("1f01"), &String::from("7468"));
        assert_eq!(output, "6b69");
    }

    #[test]
    fn basic() {
        let output = super::fixed_xor(&String::from("12"), &String::from("12"));
        assert_eq!(output, "00");
    }
}

#[cfg(test)]
mod c5_tests {

    #[test]
    fn test_newline() {
        let s = super::repeating_xor(&String::from("\n"), &String::from("ICE"));        
        assert_eq!(s, "43");
    }

    #[test]
    // why does the original not work? There are a couple of extra 0s which I don't understand
    fn test() {
        let s = super::repeating_xor(&String::from("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"), &String::from("ICE"));        
        assert_eq!(s, "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
    }
}

#[cfg(test)]
mod c6_tests {
    #[test]
    fn simple() {
        let s = super::hamming_distance("this is a test","wokka wokka!!!");
        assert_eq!(s,37);
    }
}

mod input_helpers;

static TABLE: &'static [char] = &['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',
                                  'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z',
                                  '0','1','2','3','4','5','6','7','8','9','+','/'];

fn find_index_in_table(_input: u8) -> u32 {
    for i in 0..TABLE.len() {
        if _input == TABLE[i] as u8 {
            return i as u32
        }
    }
    assert!(false);
    0
}

fn decode_from_base64(_input: &[u8], _output: &mut String) {
    let _byte_0 = _input[0] << 2;
    let _byte_1 = _input[1];
    match _input.len() {
        2 => {
            _output.push(_byte_0 as char);
            return;
        },
        3 => {
            _output.push(_byte_0 as char);
            return;
        },
        4 => {

        },
        _ => assert!(false)
    }
}

fn encode_to_base64(_input: &[u8], _output: &mut String) {
    _output.push(TABLE[((_input[0] & 0xfc) >> 2) as usize]);
    match _input.len() {
        1 => {
            _output.push(TABLE[((_input[0] & 0x3) << 4) as usize]);
            _output.push('=');
            _output.push('=');
        },
        2 => {
            _output.push(TABLE[((_input[0] & 0x3) << 4 | (_input[1] & 0xf0) >> 4) as usize]);
            _output.push(TABLE[((_input[1] & 0xf) << 2) as usize]);
            _output.push('=');
        },
        3 => {
            _output.push(TABLE[((_input[0] & 0x3) << 4 | (_input[1] & 0xf0) >> 4) as usize]);
            _output.push(TABLE[((_input[1] & 0xf) << 2 | (_input[2] & 0xfc) >> 6) as usize]);
            _output.push(TABLE[(_input[2] & 0x3f) as usize]);
        }
        _ => println!("Unsupported length {}", _input.len()),
    }
}

pub fn hex_to_base64(_input: &str) -> String {
    let mut current_index = 0;
    let _input_u8 = input_helpers::hex_string_to_u8(_input);

    let mut _output = String::from("");
    loop {
        match _input_u8.len() - current_index {
            0 => return _output,
            1 => {
                encode_to_base64(&_input_u8[current_index..current_index+1], &mut _output);
                return _output
            },
            2 => {
                encode_to_base64(&_input_u8[current_index..current_index+2], &mut _output);
                return _output
            },
            _ => {
                encode_to_base64(&_input_u8[current_index..current_index+3], &mut _output);
                current_index = current_index + 3;
            }
        };
    };
}

pub fn base_64_to_hex(_input: &str) -> String {
    let mut current_index = 0;
    let _input_u8 = _input.as_bytes();

    let mut _output = String::new();
    loop {
        if _input_u8[current_index+2] == ('=' as u8) {
            decode_from_base64(&_input_u8[current_index..current_index+2], &mut _output);
            return _output
        }
        if _input_u8[current_index+3] == ('=' as u8) {
            decode_from_base64(&_input_u8[current_index..current_index+1], &mut _output);
            return _output
        }
        decode_from_base64(&_input_u8[current_index..current_index+3], &mut _output);
        current_index = current_index + 3;
    }
}

pub fn fixed_xor(_lhs: &str, _rhs: &str) -> String {
    let _lhs_hex = input_helpers::hex_string_to_u8(_lhs);
    let _rhs_hex = input_helpers::hex_string_to_u8(_rhs);

    let res = _lhs_hex.iter()
            .zip(_rhs_hex.iter())
            .map(|(x,y)| (x ^ y))
            .collect();
    input_helpers::u8_to_hex_string(&res)
}

pub fn fixed_xor_from_u8(_lhs: &Vec<u8>, _rhs: &Vec<u8>) -> Vec<u8> {

    let res = _lhs.iter()
            .zip(_rhs.iter())
            .map(|(x,y)| (x ^ y))
            .collect();
    res
}

pub fn fixed_xor_from_u8_slice(_lhs: &[u8], _rhs: &Vec<u8>) -> Vec<u8> {
    let res = _lhs.iter()
            .zip(_rhs.iter())
            .map(|(x,y)| (x ^ y))
            .collect();
    res
}

pub fn repeating_xor(_human_string: &str, _mask: &str) -> String {
    // convert _human_string to hex
    let _input = _human_string.as_bytes();
    let _mask_bytes = _mask.as_bytes();

    // convert _mask to repeating mask in hex
    let _repetitions = _input.len() / _mask.len();
    let _remainder = _input.len() % _mask.len();

    let mut _full_mask : Vec<u8> = Vec::new();
    for _i in 0.._repetitions {
        _full_mask.extend_from_slice(& _mask_bytes);
    }
    for _i in 0.._remainder {
        _full_mask.push(_mask_bytes[_i])
    }
    input_helpers::u8_to_hex_string(&fixed_xor_from_u8_slice(_input, &_full_mask))
}

fn count_bits(_byte: u8) -> i32 {
    let mut ret : u8 = 0;
    for _i in 0..8 {
        ret += (_byte >> _i) & 1;
    }

    ret as i32
}

fn hamming_distance(_lhs: &str, _rhs: &str) -> i32 {
    let _lhs_bytes: &[u8] = _lhs.as_bytes();
    let _rhs_bytes: &[u8] = _rhs.as_bytes();

    let mut _res : i32 = 0;
    for _v in _lhs_bytes.iter()
                        .zip(_rhs_bytes.iter())
                        .map(|(x,y)| count_bits(x^y))
                        .collect::<Vec<i32>>() {
                            _res += _v;
                        }
    _res
}

fn break_vigenere_cipher(_input: &str) {
    // base64 decode

    let mut _keysize = 2;

    for _keysize in 2..41 {

    }

    // hex to string
}

mod sentences {

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
        // commenting this one out as it's super slow
        // #[test]
        // fn test() {
        //     let _s = super::find_sentence();
        //     assert_eq!(_s, "Now that the party is jumping\n");
        // }
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
            let _input_hex = super::input_helpers::hex_string_to_u8(_input);
            let _candidate = super::fixed_xor_from_u8(&_input_hex, &_mask);
            let _score = score_candidate(&_candidate);
            if _score > _best_score {
                _best_score = _score;
                _best_string = super::input_helpers::u8_to_string(&_candidate);
            }
        }

        (_best_string, _best_score)
    }

    pub fn find_sentence() -> String {
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
        _best_string
    //    println!("{} -> {}", _best_string, _best_score);
    }
}
