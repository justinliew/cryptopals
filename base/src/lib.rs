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
        assert_eq!(output, "0");
    }
}

mod c3_tests {
    #[test]
    fn test() {
        let ret = super::get_best_candidate_sentence(&String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"));
        println!("{}", ret);
    }
}

mod input_helpers;

static TABLE: &'static [char] = &['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',
                                  'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z',
                                  '0','1','2','3','4','5','6','7','8','9','+','/'];

fn encode(_input: &[u8], _output: &mut String) {
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
                encode(&_input_u8[current_index..current_index+1], &mut _output);
                return _output
            },
            2 => {
                encode(&_input_u8[current_index..current_index+2], &mut _output);
                return _output
            },
            _ => {
                encode(&_input_u8[current_index..current_index+3], &mut _output);
                current_index = current_index + 3;
            }
        };
    };
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

pub fn fixed_xor_from_u8(_lhs: &Vec<u8>, _rhs: &Vec<u8>) -> String {

    let res = _lhs.iter()
            .zip(_rhs.iter())
            .map(|(x,y)| (x ^ y))
            .collect();
    input_helpers::u8_to_hex_string(&res)
}

pub fn get_best_candidate_sentence(_input: &str) -> String {
    for _c in 1..256 {
        let mut _mask = vec![];
        for _i in 0.._input.len() {
            _mask.push(_c as u8);
        }
        let _input_hex = input_helpers::hex_string_to_u8(_input);
        let _candidate = fixed_xor_from_u8(&_input_hex, &_mask);
        println!("C: {}", _candidate);
    }

    String::new()
}