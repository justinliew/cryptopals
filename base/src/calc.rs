use convert;
use decode;

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
    fn test() {
        let s = super::repeating_xor(&String::from("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"), &String::from("ICE"));        
        assert_eq!(s, "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
    }
}

static TABLE: &'static [char] = &['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',
                                  'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z',
                                  '0','1','2','3','4','5','6','7','8','9','+','/'];

fn find_index_in_table(_input: u8) -> u8 {
    for i in 0..TABLE.len() {
        if _input == (TABLE[i] as u8) {
            return i as u8
        }
    }
    assert!(false);
    0
}

pub fn hex_to_base64(_input: &str) -> String {
    let mut current_index = 0;
    let _input_u8 = convert::hex_string_to_u8(_input);

    let mut _output = String::from("");
    loop {
        match _input_u8.len() - current_index {
            0 => return _output,
            1 => {
                _output.push(TABLE[((_input_u8[current_index] & 0xfc) >> 2) as usize]);
                _output.push(TABLE[((_input_u8[current_index] & 0x3) << 4) as usize]);
                _output.push('=');
                _output.push('=');
                return _output
            },
            2 => {
                _output.push(TABLE[((_input_u8[current_index] & 0xfc) >> 2) as usize]);
                _output.push(TABLE[((_input_u8[current_index] & 0x3) << 4 | (_input_u8[current_index+1] & 0xf0) >> 4) as usize]);
                _output.push(TABLE[((_input_u8[current_index+1] & 0xf) << 2) as usize]);
                _output.push('=');
                return _output
            },
            _ => {
                _output.push(TABLE[((_input_u8[current_index] & 0xfc) >> 2) as usize]);
                _output.push(TABLE[((_input_u8[current_index] & 0x3) << 4 | (_input_u8[current_index+1] & 0xf0) >> 4) as usize]);
                _output.push(TABLE[((_input_u8[current_index+1] & 0xf) << 2 | (_input_u8[current_index+2] & 0xfc) >> 6) as usize]);
                _output.push(TABLE[(_input_u8[current_index+2] & 0x3f) as usize]);
                current_index = current_index + 3;
            }
        };
    };
}

fn decode_from_base64(_input: &[u8], _output: &mut Vec<u8>) {
    let _decoded_0 = find_index_in_table(_input[0]);
    let _decoded_1 = find_index_in_table(_input[1]);
    let _byte_0 = (_decoded_0 << 2) | (_decoded_1 >> 4);
    match _input.len() {
        2 => {
            _output.push(_byte_0);
            return;
        },
        3 => {
            let _decoded_2 = find_index_in_table(_input[2]);
            let _byte_1 = ((_decoded_1 >> 4) & 0xf0) | (_decoded_2 >> 2);
            _output.push(_byte_0);
            _output.push(_byte_1);
            return;
        },
        4 => {
            let _decoded_2 = find_index_in_table(_input[2]);
            let _decoded_3 = find_index_in_table(_input[3]);
            let _byte_1 = ((_decoded_1 >> 4) & 0xf0) | (_decoded_2 >> 2);
            let _byte_2 = ((_decoded_2 >> 6) & 0xc0) | (_decoded_3 & 0x3f);
            _output.push(_byte_0);
            _output.push(_byte_1);
            _output.push(_byte_2);
        },
        _ => assert!(false)
    }
}

pub fn decode_base64_to_bytes(_input: &str) -> Vec<u8> {
    let mut current_index = 0;
    let _input_u8 = _input.as_bytes();

    let mut _output : Vec<u8> = Vec::new();
    loop {
        if _input_u8[current_index+2] == ('=' as u8) {
            decode_from_base64(&_input_u8[current_index..current_index+2], &mut _output);
            return _output
        }
        if _input_u8[current_index+3] == ('=' as u8) {
            decode_from_base64(&_input_u8[current_index..current_index+3], &mut _output);
            return _output
        }
        decode_from_base64(&_input_u8[current_index..current_index+4], &mut _output);
        current_index = current_index + 4;
        if current_index >= _input_u8.len() {
            return _output
        }
    }
}

pub fn fixed_xor(_lhs: &str, _rhs: &str) -> String {
    let _lhs_hex = convert::hex_string_to_u8(_lhs);
    let _rhs_hex = convert::hex_string_to_u8(_rhs);

    let res = _lhs_hex.iter()
            .zip(_rhs_hex.iter())
            .map(|(x,y)| (x ^ y))
            .collect();
    convert::u8_to_hex_string(&res)
}

pub fn repeating_xor(_human_string: &str, _mask: &str) -> String {
    let _mask_bytes = _mask.as_bytes();
    let _output = repeating_xor_from_bytes(_human_string, _mask_bytes);
    convert::u8_to_hex_string(&_output)
}

pub fn repeating_xor_from_bytes(_human_string: &str, _mask_bytes: &[u8]) -> Vec<u8> {
    // convert _human_string to hex
    let _input = _human_string.as_bytes();

    // convert _mask to repeating mask in hex
    let _repetitions = _input.len() / _mask_bytes.len();
    let _remainder = _input.len() % _mask_bytes.len();

    let mut _full_mask : Vec<u8> = Vec::new();
    for _i in 0.._repetitions {
        _full_mask.extend_from_slice(& _mask_bytes);
    }
    for _i in 0.._remainder {
        _full_mask.push(_mask_bytes[_i])
    }
    convert::fixed_xor_from_u8_slice(_input, &_full_mask)
}

fn count_bits(_byte: u8) -> i32 {
    let mut ret : u8 = 0;
    for _i in 0..8 {
        ret += (_byte >> _i) & 1;
    }

    ret as i32
}

fn hamming_distance(_lhs: &[u8], _rhs: &[u8]) -> i32 {
    let mut _res : i32 = 0;
    for _v in _lhs.iter()
                        .zip(_rhs.iter())
                        .map(|(x,y)| count_bits(x^y))
                        .collect::<Vec<i32>>() {
                            _res += _v;
                        }
    _res
}

fn transpose_and_test(_input: &[u8], _keysize: i32) -> Vec<u8> {
    let end = _input.len() as i32 - _keysize;
    let mut blocks : Vec<Vec<u8>> = Vec::new(); 
    for i in 0.._keysize {
        blocks.push(Vec::new());
    }

    for i in 0..end {
        let block_num = i % _keysize;
        blocks[block_num as usize].push(_input[i as usize]);
    }

    let mut _output : Vec<u8> = Vec::new();

    for i in 0..blocks.len() {
        let (_,_,_key_elem) = decode::get_best_candidate_sentence_from_hex_bytes(&blocks[i as usize]);
        _output.push(_key_elem);
    }
    _output
}

pub fn break_vigenere_cipher(_input: &str) {

    let _input_bytes = decode_base64_to_bytes(&_input.replace("\n","")); 

    let mut _keysize = 2;

    let mut _best_distance = 99999;
    let mut _best_distance_key_size : i32 = 2;
    for _keysize in 2..41 {
        let distance1 = hamming_distance(&_input_bytes[0.._keysize], &_input_bytes[_keysize.._keysize*2]) / (_keysize as i32);
        let distance2 = hamming_distance(&_input_bytes[_keysize*2.._keysize*3], &_input_bytes[_keysize*3.._keysize*4]) / (_keysize as i32);
        let distance = (distance1 + distance2) / 2;
        if distance < _best_distance {
            _best_distance = distance;
            _best_distance_key_size = _keysize as i32;
        }
    }

    let _key = transpose_and_test(&_input_bytes, _best_distance_key_size);
    println!("Possible best keysize is {}; key is {:?}", _best_distance_key_size, _key);

    let _decoded = repeating_xor_from_bytes(&_input, &_key);
    let _decoded_string = convert::u8_to_string(&_decoded);
    println!("Decoded string: {}", _decoded_string);

}
