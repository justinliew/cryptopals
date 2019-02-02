extern crate crypto;

#[cfg(test)]
mod c9_tests {
    #[test]
    fn pad1() {
        let s = String::from("YELLOW SUBMARINE");
        let i = 20;

        let res = super::pkcs7_pad(&s,i);
        assert_eq!(res, "YELLOW SUBMARINE\x04\x04\x04\x04");
    }

    #[test]
    fn pad2() {
        let s = String::from("Justin Liew Justin Liew");
        let i = 32;

        let res = super::pkcs7_pad(&s,i);
        assert_eq!(res, "Justin Liew Justin Liew\x09\x09\x09\x09\x09\x09\x09\x09\x09");
    }

    #[test]
    fn no_pad() {
        let s = String::from("four");
        let i = 4;

        let res = super::pkcs7_pad(&s,i);
        assert_eq!(s, res);
    }

    #[test]
    fn pad_too_short() {
        let s = String::from("Justin Liew");
        let i = 5;

        let res = super::pkcs7_pad(&s,i);
        assert_eq!(res, s);
    }
}

#[cfg(test)]
mod c10_tests {
    #[test]
    fn basic_cbc_encrypt() {
        let enc = super::cbc::encrypt("YELLOW SUBMARINE", "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00", "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
        assert_eq!(enc, "YELLOW SUBMARINE");
    }

    #[test]
    fn two_block_cbc_encrypt() {

    }
}

fn pkcs7_pad(s : &str, len: usize) -> String {
    if s.len() >= len {
        return s.to_string()
    }
    let num_bytes : u8 = len as u8 - s.len() as u8;
    let mut ret = String::from(s);
    for _ in 0..num_bytes {
        ret.push(num_bytes as char);
    }
    ret
}

// TODO break plaintext into blocks of 16
// TODO pad last block
// TODO xor + encrypt
pub mod cbc {
    pub fn encrypt(key: &str, plaintext: &str, iv: &str) -> std::Vec<u8> {
        let mut encryptor = crypto::aes::ecb_encryptor(crypto::aes::KeySize::KeySize128, &key.as_bytes(), crypto::blockmodes::NoPadding);

        let mut res = String::from("");
        for _ in 0..plaintext.len()/16 {
            let plain = plaintext.as_bytes();
            let mut enc = vec![0u8; plain.len()];
            {
                let mut read_buf = crypto::buffer::RefReadBuffer::new(&plain);
                let mut write_buf = crypto::buffer::RefWriteBuffer::new(&mut enc);
                encryptor.encrypt(&mut read_buf, &mut write_buf, true);
            }
            let s = String::from_utf8(enc).expect("Didn't find a UTF8 string");
            res.push_str(&s);
        }
        res
    }

    // TODO break ciphertext into blocks of 16
    // TODO decrypt
    // TODO last block
    pub fn decrypt(key: &str, ciphertext: &str, iv: &str) -> String {
        let mut decryptor = crypto::aes::ecb_decryptor(crypto::aes::KeySize::KeySize128, &key.as_bytes(), crypto::blockmodes::NoPadding);

        let enc = ciphertext.as_bytes();
        let mut dec = vec![0u8; enc.len()];
        {
            let mut read_buf = crypto::buffer::RefReadBuffer::new(&enc);
            let mut write_buf = crypto::buffer::RefWriteBuffer::new(&mut dec);
            decryptor.decrypt(&mut read_buf, &mut write_buf, true);
        }

        "".to_string()
    }
}

