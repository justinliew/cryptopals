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

fn cbc(key: &str, ciphertext: &str, iv: &str) -> String {
    let key = String::from("YELLOW SUBMARINE");
    let mut decryptor = aes::ecb_decryptor(aes::KeySize::KeySize128, &key.as_bytes(), blockmodes::NoPadding);

    let mut f = File::open("7.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let enc = base::calc::decode_base64_to_bytes(&contents);
    let mut dec = vec![0u8; enc.len()];

    {
        let mut read_buf = buffer::RefReadBuffer::new(&enc);
        let mut write_buf = buffer::RefWriteBuffer::new(&mut dec);
        decryptor.decrypt(&mut read_buf, &mut write_buf, true);
    }

    let s = base::convert::u8_to_string(&dec);
    println!("{}", s);
}

}