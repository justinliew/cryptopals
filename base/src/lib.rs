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
    // #[test]
    // fn example() {
    //     let output = super::fixed_xor(&String::from("1c0111001f010100061a024b53535009181c"), &String::from("686974207468652062756c6c277320657965"));
    //     assert_eq!(output, "746865206b696420646f6e277420706c6179");
    // }

    #[test]
    fn basic() {
        let output = super::fixed_xor(&String::from("12"), &String::from("12"));
        assert_eq!(output, "0");
    }
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

fn bytes_to_hex_digit(b : &str) -> u8 {
    u8::from_str_radix(b, 16).unwrap()
}

fn string_to_hex(_input: &str) -> Vec<u8> {
    let mut _stream = String::new();
    if _input.len() % 2 != 0 {
        _stream.push_str("0");
    }
    _stream.push_str(_input);

    let mut _output = Vec::new();
    let mut _i = 0;
    while _i < _stream.len() {
        _output.push(bytes_to_hex_digit(&_stream[_i.._i+2]));
        _i += 2;
    }
    _output
}

fn hex_to_string(_input: &Vec<u8>) -> String {
    let _output = String::new();

    for _i in _input.iter() {
        // TODO
//        _output.push(_i.to_string());
    }
    _output
}

pub fn hex_to_base64(_input: &str) -> String {
    let mut current_index = 0;
    let _input_hex = string_to_hex(_input);

    let mut _output = String::from("");
    loop {
        match _input_hex.len() - current_index {
            0 => return _output,
            1 => {
                encode(&_input_hex[current_index..current_index+1], &mut _output);
                return _output
            },
            2 => {
                encode(&_input_hex[current_index..current_index+2], &mut _output);
                return _output
            },
            _ => {
                encode(&_input_hex[current_index..current_index+3], &mut _output);
                current_index = current_index + 3;
            }
        };
    };
}

fn xor(_lhs: &str, _rhs: &str) -> String {
    let _lhs_hex = string_to_hex(_lhs);
    let _rhs_hex = string_to_hex(_rhs);

    let mut _output_bytes = Vec::new();
    let _zip_iter = _lhs_hex.iter().zip(_rhs_hex.iter());
    for (x,y) in _zip_iter {
        _output_bytes.push(x ^ y);
    }

    // this should be the goal but I need to break it up in order to understand and debug it
    // _lhs_bytes.iter()
    //             .zip(_rhs_bytes.iter())
    //             .map(|(x,y)| (x ^ y));

//    String::from_utf8(_output_bytes).unwrap()
    hex_to_string(&_output_bytes)
}

pub fn fixed_xor(_lhs: &str, _rhs: &str) -> String {
    xor(&_lhs,&_rhs)
}