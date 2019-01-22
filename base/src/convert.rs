pub fn hex_string_to_u8(input: &str) -> Vec<u8> {
    let mut stream = String::new();
    if input.len() % 2 != 0 {
        stream.push_str("0");
    }
    stream.push_str(input);

    let mut output = Vec::new();
    let mut i = 0;
    while i < stream.len() {
        output.push(u8::from_str_radix(&stream[i..i+2],16).unwrap());
        i += 2;
    }
    output
}

static HEX_TABLE: &'static [char] = &['0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f'];

pub fn u8_to_hex_string(input: &Vec<u8>) -> String {
    let mut output = String::new();

    for i in input.iter() {

        let mut quotient : u8 = *i;
        let mut substring = String::new();
        loop {
            let remainder = quotient % 16;
            quotient = quotient / 16;
            substring.push(HEX_TABLE[remainder as usize]);
            if quotient == 0 {
                break
            }
        }
        if substring.len() == 1 {
            substring.push('0');
        }
        output.push_str(&substring.chars().rev().collect::<String>());
    }

    let mut ret = String::new();
    if output.len() % 2 != 0 {
        ret.push_str("0");
    }
    ret.push_str(&output);
    ret
}

pub fn u8_to_string(_input: &Vec<u8>) -> String {
    let mut _output = String::new();
    for _i in _input.iter() {
        _output.push(*_i as char);
    }
    _output
}

pub fn string_to_u8(_input: &str) -> Vec<u8> {
    let mut _output : Vec<u8> = Vec::new();

    for _i in _input.chars() {
        _output.push(_i as u8);
    }
    _output
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