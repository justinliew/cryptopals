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

pub fn decimal_to_hex_string(mut _input: u8) -> String {
    let mut _substring = String::new();
    loop {
        let _remainder = _input % 16;
        _input = _input / 16;

        _substring.push(HEX_TABLE[_remainder as usize]);
        if _input == 0 {
            break
        }
    }
    _substring.chars().rev().collect::<String>()
}

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
        if _substring.len() == 1 {
            _substring.push('0');
        }
        _output.push_str(&_substring.chars().rev().collect::<String>());
    }

    let mut _ret = String::new();
    if _output.len() % 2 != 0 {
        _ret.push_str("0");
    }
    _ret.push_str(&_output);
    _ret
}

pub fn u8_to_string(_input: &Vec<u8>) -> String {
    let mut _output = String::new();
    for _i in _input.iter() {
        _output.push(*_i as char);
    }
    _output
}