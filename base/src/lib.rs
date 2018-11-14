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
        let output = super::fixed_xor(&String::from("1"), &String::from("1"));
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

pub fn hex_to_base64(_input: &str) -> String {
    let mut _num_bytes = _input.len();
    let mut _stream = String::new();
    if _num_bytes % 2 != 0 {
        _stream.push_str("0");
        _num_bytes = _num_bytes + 1;
    }

    _stream.push_str(_input);
    let mut current_index = 0;

    let mut _output = String::from("");
    loop {
        match _num_bytes - current_index {
            0 => return _output,
            1 | 3 | 5 => {
                println!("Unsupported bytes");
            },
            2 => {
                let mut x = Vec::new();
                x.push(bytes_to_hex_digit(&_stream[current_index..current_index+2]));
                encode(&x, &mut _output);
                return _output
            },
            4 => {
                let mut x = Vec::new();
                x.push(bytes_to_hex_digit(&_stream[current_index..current_index+2]));
                x.push(bytes_to_hex_digit(&_stream[current_index+2..current_index+4]));
                encode(&x, &mut _output);
                return _output
            },
            _ => {
                let mut x = Vec::new();
                x.push(bytes_to_hex_digit(&_stream[current_index..current_index+2]));
                x.push(bytes_to_hex_digit(&_stream[current_index+2..current_index+4]));
                x.push(bytes_to_hex_digit(&_stream[current_index+4..current_index+6]));
                encode(&x, &mut _output);
            }
        };
        current_index = current_index + 6;
    };
}

fn xor(_lhs: &str, _rhs: &str) -> String {
    let _lhs_bytes = _lhs.as_bytes();
    let _rhs_bytes = _rhs.as_bytes();
    println!("LEN: {}, LHS: {}, RHS: {}, XOR: {}", _lhs_bytes.len(), _lhs_bytes[0], _rhs_bytes[0], _lhs_bytes[0] ^ _rhs_bytes[0]);

    let mut _output_bytes = Vec::new();
    let _zip_iter = _lhs_bytes.iter().zip(_rhs_bytes.iter());
    for (x,y) in _zip_iter {
        println!("{} xor {} = {}", x,y,x^y);
        _output_bytes.push(x ^ y);
    }

    // this should be the goal but I need to break it up in order to understand and debug it
    // _lhs_bytes.iter()
    //             .zip(_rhs_bytes.iter())
    //             .map(|(x,y)| (x ^ y));

    String::from("unimplemented")
}

pub fn fixed_xor(_lhs: &str, _rhs: &str) -> String {
    let _lhs_decoded = hex_to_base64(_lhs);
    let _rhs_decoded = hex_to_base64(_rhs);
    assert_eq!(_lhs_decoded.len(), _rhs_decoded.len());
    xor(&_lhs_decoded,&_rhs_decoded)
}