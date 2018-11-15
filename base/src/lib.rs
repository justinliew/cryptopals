#[cfg(test)]
mod basic_tests {
    #[test]
    fn string_to_hex_and_back() {
        let transformed = super::u8_to_hex_string(&super::hex_string_to_u8("123456789abcdef"));
        assert_eq!(transformed, "123456789abcdef");
    }
}

#[cfg(test)]
mod c1_tests {
    // #[test]
    // fn example() {
    //     let output = super::hex_to_base64(&String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"));
    //     assert_eq!(output, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    // }

    // #[test]
    // fn single() {
    //     let output = super::hex_to_base64(&String::from("1"));
    //     assert_eq!(output, "AQ==");
    // }

    // #[test]
    // fn double() {
    //     let output = super::hex_to_base64(&String::from("12"));
    //     assert_eq!(output, "Eg==");
    // }

    // #[test]
    // fn simplest_example() {
    //     let output = super::hex_to_base64(&String::from("010101"));
    //     assert_eq!(output, "AQEB");
    // }
}

#[cfg(test)]
mod c2_tests {
    // #[test]
    // fn example() {
    //     let output = super::fixed_xor(&String::from("1c0111001f010100061a024b53535009181c"), &String::from("686974207468652062756c6c277320657965"));
    //     assert_eq!(output, "746865206b696420646f6e277420706c6179");
    // }

    // #[test]
    // fn start_of_example() {
    //     let output = super::fixed_xor(&String::from("1c01"), &String::from("6869"));
    //     assert_eq!(output, "7468");
    // }

    // #[test]
    // fn basic() {
    //     let output = super::fixed_xor(&String::from("12"), &String::from("12"));
    //     assert_eq!(output, "0");
    // }
}

fn hex_string_to_u8(_input: &str) -> Vec<u8> {
    let mut _stream = String::new();
    if _input.len() % 2 != 0 {
        _stream.push_str("0");
    }
    _stream.push_str(_input);

    let mut _output = Vec::new();
    let mut _i = 0;
    while _i < _stream.len() {
        _output.push(u8::from_str_radix(&_stream[_i.._i+2],16).unwrap());
        _i += 2;
    }
    _output
}

static HEX_TABLE: &'static [char] = &['0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f'];

fn u8_to_hex_string(_input: &Vec<u8>) -> String {
    let mut _output = String::new();

    for _i in _input.iter() {

        let mut _quotient : u8 = *_i;
        println!("Initial Q: {}", _quotient);
        loop {
            let _remainder = _quotient % 16;
            _quotient = _quotient / 16;
            println!("Q: {}, R: {}", _quotient, HEX_TABLE[_remainder as usize]);
            _output.push_str(&*_remainder.to_string());
            if _quotient == 0 {
                break
            }
        }
    }
    _output
}

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
    let _input_u8 = hex_string_to_u8(_input);

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
    let _lhs_hex = hex_string_to_u8(_lhs);
    let _rhs_hex = hex_string_to_u8(_rhs);

    let _zip_iter = _lhs_hex.iter().zip(_rhs_hex.iter());
    for (x,y) in _zip_iter {
        println!("X: {}, Y: {}, XOR: {}", x, y, x^y);
    }

    // this should be the goal but I need to break it up in order to understand and debug it
    let res = _lhs_hex.iter()
            .zip(_rhs_hex.iter())
            .map(|(x,y)| (x ^ y))
            .collect();
    u8_to_hex_string(&res)
}