pub fn hex_string_to_u8(_input: &str) -> Vec<u8> {
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

pub fn u8_to_hex_string(_input: &Vec<u8>) -> String {
    let mut _output = String::new();

    for _i in _input.iter() {

        let mut _quotient : u8 = *_i;
        let mut _substring = String::new();
        loop {
            let _remainder = _quotient % 16;
            _quotient = _quotient / 16;
            _substring.push(HEX_TABLE[_remainder as usize]);
            if _quotient == 0 {
                break
            }
        }
        ;
        _output.push_str(&_substring.chars().rev().collect::<String>());
    }
    _output
}