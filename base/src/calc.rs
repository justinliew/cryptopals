use convert;
use decode;

#[cfg(test)]
mod c1_tests {
    #[test]
    fn example() {
        let output = super::hex_string_to_base64_string(&String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"));
        assert_eq!(output, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }

    #[test]
    fn single() {
        let output = super::hex_string_to_base64_string(&String::from("1"));
        assert_eq!(output, "AQ==");
    }

    #[test]
    fn double() {
        let output = super::hex_string_to_base64_string(&String::from("12"));
        assert_eq!(output, "Eg==");
    }

    #[test]
    fn simplest_example() {
        let output = super::hex_string_to_base64_string(&String::from("010101"));
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
        let s = super::repeating_xor_to_hex_string(&String::from("\n"), &String::from("ICE"));        
        assert_eq!(s, "43");
    }

    #[test]
    fn test() {
        let s = super::repeating_xor_to_hex_string(&String::from("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"), &String::from("ICE"));        
        assert_eq!(s, "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
    }
}

#[cfg(test)]
mod c6_tests {
    
    #[test]
    fn simple() {
        let original = super::hex_string_to_base64_string(&String::from("a"));
        println!("Original: {}", original);
        let decoded = super::decode_base64_to_bytes(&original);
        let hex_string = super::convert::u8_to_hex_string(&decoded);
        println!("Decoded: {}", hex_string);
    }

    #[test]
    fn prebaked() {
        // we want to generate something here and ensure it gets back.
        let original_text = "fuse fuel for falling flocks";
        let key = "few";
        let cipher_bytes = super::repeating_xor(&original_text, &key);
        println!("Cipher Bytes: {:?}", cipher_bytes);
        super::break_vigenere_cipher(&cipher_bytes);
    }
}

static TABLE: &'static [char] = &['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',
                                  'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z',
                                  '0','1','2','3','4','5','6','7','8','9','+','/'];

fn find_index_in_table(input: u8) -> u8 {
    for i in 0..TABLE.len() {
        if input == (TABLE[i] as u8) {
            return i as u8
        }
    }
    println!("Couldn't find {}", input);
    assert!(false);
    0
}

pub fn hex_string_to_base64_string(input: &str) -> String {
    let mut current_index = 0;
    let input_u8 = convert::hex_string_to_u8(input);

    let mut output = String::from("");
    loop {
        match input_u8.len() - current_index {
            0 => return output,
            1 => {
                output.push(TABLE[((input_u8[current_index] & 0xfc) >> 2) as usize]);
                output.push(TABLE[((input_u8[current_index] & 0x3) << 4) as usize]);
                output.push('=');
                output.push('=');
                return output
            },
            2 => {
                output.push(TABLE[((input_u8[current_index] & 0xfc) >> 2) as usize]);
                output.push(TABLE[((input_u8[current_index] & 0x3) << 4 | (input_u8[current_index+1] & 0xf0) >> 4) as usize]);
                output.push(TABLE[((input_u8[current_index+1] & 0xf) << 2) as usize]);
                output.push('=');
                return output
            },
            _ => {
                output.push(TABLE[((input_u8[current_index] & 0xfc) >> 2) as usize]);
                output.push(TABLE[((input_u8[current_index] & 0x3) << 4 | (input_u8[current_index+1] & 0xf0) >> 4) as usize]);
                output.push(TABLE[((input_u8[current_index+1] & 0xf) << 2 | (input_u8[current_index+2] & 0xfc) >> 6) as usize]);
                output.push(TABLE[(input_u8[current_index+2] & 0x3f) as usize]);
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

pub fn decode_base64_to_bytes(input: &str) -> Vec<u8> {
    let mut current_index = 0;

    let stripped : String = input.chars().filter(|&c| c != '\n').collect();
    let input_u8 = stripped.as_bytes();

    let mut output : Vec<u8> = Vec::new();
    loop {
        if input_u8[current_index+2] == ('=' as u8) {
            decode_from_base64(&input_u8[current_index..current_index+2], &mut output);
            return output
        }
        if input_u8[current_index+3] == ('=' as u8) {
            decode_from_base64(&input_u8[current_index..current_index+3], &mut output);
            return output
        }
        decode_from_base64(&input_u8[current_index..current_index+4], &mut output);
        current_index = current_index + 4;
        if current_index >= input_u8.len() {
            return output
        }
    }
}

pub fn fixed_xor(lhs: &str, rhs: &str) -> String {
    let lhs_hex = convert::hex_string_to_u8(lhs);
    let rhs_hex = convert::hex_string_to_u8(rhs);

    let res = lhs_hex.iter()
            .zip(rhs_hex.iter())
            .map(|(x,y)| (x ^ y))
            .collect();
    convert::u8_to_hex_string(&res)
}

pub fn repeating_xor_to_hex_string(human_string: &str, mask: &str) -> String {
    let output = repeating_xor(human_string, mask);
    convert::u8_to_hex_string(&output)
}

pub fn repeating_xor(human_string: &str, mask: &str) -> Vec<u8> {
    let mask_bytes = mask.as_bytes();
    let input = human_string.as_bytes();
    repeating_xor_from_bytes(input, mask_bytes)
}

pub fn repeating_xor_from_bytes(input: &[u8], mask_bytes: &[u8]) -> Vec<u8> {

    // convert _mask to repeating mask in hex
    let repetitions = input.len() / mask_bytes.len();
    let remainder = input.len() % mask_bytes.len();

    let mut full_mask : Vec<u8> = Vec::new();
    for _ in 0..repetitions {
        full_mask.extend_from_slice(&mask_bytes);
    }
    for i in 0..remainder {
        full_mask.push(mask_bytes[i])
    }
    convert::fixed_xor_from_u8_slice(input, &full_mask)
}

fn count_bits(byte: u8) -> i32 {
    let mut ret : u8 = 0;
    for i in 0..8 {
        ret += (byte >> i) & 1;
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

fn transpose_and_test(input: &[u8], keysize: i32) -> Vec<u8> {
    let end = input.len() as i32 - keysize;
    let mut blocks : Vec<Vec<u8>> = Vec::new(); 
    for _ in 0..keysize {
        blocks.push(Vec::new());
    }

    for i in 0..end {
        let block_num = i % keysize;
        blocks[block_num as usize].push(input[i as usize]);
    }

    let mut output : Vec<u8> = Vec::new();

    for i in 0..blocks.len() {
        let (_,_,key_elem) = decode::get_best_candidate_sentence_from_hex_bytes(&blocks[i as usize]);
        output.push(key_elem);
    }
    output
}

pub fn break_vigenere_cipher_base64(input_base64: &str) {

    let input_bytes = decode_base64_to_bytes(&input_base64); 
    break_vigenere_cipher(&input_bytes);
}

// https://trustedsignal.blogspot.com/2015/06/xord-play-normalized-hamming-distance.html
// https://trustedsignal.blogspot.com/2015/07/cracking-repeating-xor-key-crypto.html
pub fn break_vigenere_cipher(input_bytes: &Vec<u8>) {

    let mut best_distance : f32 = 99999.;
    let mut best_distance_key_size : i32 = 2;
    for keysize in 2..41 {

        if keysize > input_bytes.len() / 2 {
            break
        }

        let mut distances = vec![];
        for group in 0..(input_bytes.len()/keysize-1) {
            let lower = group*keysize;
            let d = hamming_distance(&input_bytes[lower..lower+keysize], &input_bytes[lower+keysize..lower+keysize*2]);
            distances.push(d);
        }
        if distances.len() == 0 {
            continue
        }

        let mut avg : f32 = 0.0;
        for i in distances.iter() {
            avg += *i as f32;
        }
        avg = avg / keysize as f32;
        avg = avg / distances.len() as f32;
        println!("Keysize: {}, Distance: {}", keysize, avg);
        if avg < best_distance {
            best_distance = avg;
            best_distance_key_size = keysize as i32;
        }
    }

    let key = transpose_and_test(&input_bytes, best_distance_key_size);
    println!("Possible best keysize is {}; key is {:?}", best_distance_key_size, key);

    let decoded = repeating_xor_from_bytes(input_bytes, &key);
    let decoded_string = convert::u8_to_string(&decoded);
    println!("Decoded string: {}", decoded_string);

}
