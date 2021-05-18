// 8 * 3 = 24 = 4 * 6
const BASE64_CHARS: &'static [u8] = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();

pub fn base64_encode(input: &str) -> String {
    let mut out: Vec<u8> = Vec::with_capacity(input.len() * 8 / 6);
    const EQUAL: u8 = '=' as u8;
    for chunk in input.as_bytes().chunks(3) {
        let first_byte = chunk[0];
        out.push(BASE64_CHARS[((first_byte & 0b1111_1100) >> 2) as usize]);
        if chunk.len() > 1 {
            let second_byte = chunk[1];

            out.push(BASE64_CHARS[(
                    ((first_byte & 0b0000_0011) << 4) | ((second_byte & 0b1111_0000) >> 4)
            ) as usize ]);

            if chunk.len() > 2 {
                let third_byte = chunk[2];
                out.push(BASE64_CHARS[(
                        ((second_byte & 0b0000_1111) << 2) | ((third_byte & 0b1100_0000) >> 6)
                ) as usize ]);
                out.push(BASE64_CHARS[(third_byte & 0b0011_1111) as usize]);
            } else {
                out.push(BASE64_CHARS[((second_byte & 0b0000_1111) << 2) as usize]);
                out.push(EQUAL);
            }
        } else {
            out.push(BASE64_CHARS[((first_byte & 0b0000_0011) << 4) as usize]);
            out.push(EQUAL);
            out.push(EQUAL);
        }
    }

    String::from_utf8(out).unwrap()
}

fn base64_find_idx(c: u8) -> u8 {
    if c >= 'A' as u8 && c <= 'Z' as u8 {
        return c - 'A' as u8
    } else if c >= 'a' as u8 && c <= 'z' as u8{
        return c - 'a' as u8 + 26;
    } else if c >= '0' as u8 && c <= '9' as u8 {
        return c - '0' as u8 + 52;
    } else if c == '+' as u8 {
        return 62;
    } else if c == '/' as u8{
        return 63;
    } else {
        unreachable!()
    }
}

pub fn base64_decode(input: &str) -> String {
    let mut out: Vec<u8> = Vec::with_capacity(input.len() * 6 / 8);
    const EQUAL: u8 = '=' as u8;
    for chunk in input.as_bytes().chunks_exact(4) {
        if let [first_byte, second_byte, third_byte, last_byte] = chunk {
            let first_byte = base64_find_idx(*first_byte);
            let second_byte = base64_find_idx(*second_byte);

            out.push((first_byte & 0b0011_1111) << 2 | ((second_byte & 0b0011_0000) >> 4));

            if third_byte != &EQUAL {
                let third_byte = base64_find_idx(*third_byte);
                out.push(((second_byte & 0b0000_1111) << 4) | (third_byte & 0b0011_1100) >> 2);

                if last_byte != &EQUAL {
                    let last_byte = base64_find_idx(*last_byte);
                    out.push(((third_byte & 0b0000_0011) << 6) | (last_byte & 0b0011_1111));
                }
            }
        }
    }

    String::from_utf8(out).unwrap()
}

pub fn caesar_encrypt(text: &str, mut shift: i32) -> String {
    if shift < 0 { shift = shift.rem_euclid(26); }

    let mut out = Vec::with_capacity(text.len());
    for c in text.as_bytes() {
        if *c >= 'A' as u8 && *c <= 'Z' as u8 {
            let mut width = c - 'A' as u8;
            width = ((width as i32 + shift) % 26) as u8;
            out.push('A' as u8 + width);
        } else if *c >= 'a' as u8 && *c <= 'z' as u8 {
            let mut width = c - 'a' as u8;
            width = ((width as i32 + shift) % 26) as u8;
            out.push('a' as u8 + width);
        } else {
            out.push(*c);
        }
    }

    String::from_utf8(out).unwrap()
}

pub fn caesar_decrypt(text: &str, shift: i32) -> String {
    caesar_encrypt(text, -shift)
}

// https://learnblockchain.cn/2019/06/23/ecdh
// we don't have u256 in std...
pub fn elliptic_curve_key_exchange() {
    todo!()
}

// we don't have L-U decomposite
pub fn hilli_cipher() {
    todo!()
}

fn morse_in(c: char) -> &'static str {
    match c {
        'a' => ".-",
        'b' => "-...",
        'c' => "-.-.",
        'd' => "-..",
        'e' => ".",
        'f' => "..-.",
        'g' => "--.",
        'h' => "....",
        'i' => "..",
        'j' => ".---",
        'k' => "-.-",
        'l' => ".-..",
        'm' => "--",
        'n' => "-.",
        'o' => "---",
        'p' => ".--.",
        'q' => "--.-",
        'r' => ".-.",
        's' => "...",
        't' => "-",
        'u' => "..-",
        'v' => "...-",
        'w' => ".--",
        'x' => "-..-",
        'y' => "-.--",
        'z' => "--..",
        '1' => ".----",
        '2' => "..---",
        '3' => "...--",
        '4' => "....-",
        '5' => ".....",
        '6' => "-....",
        '7' => "--...",
        '8' => "---..",
        '9' => "----.",
        '0' => "-----",
        _ => unreachable!()
    }
}

fn morse_out(s: &str) -> char {
    match s {
        ".-" => 'a',
        "-..." => 'b',
        "-.-." => 'c',
        "-.." => 'd',
        "." => 'e',
        "..-." => 'f',
        "--." => 'g',
        "...." => 'h',
        ".." => 'i',
        ".---" => 'j',
        "-.-" => 'k',
        ".-.." => 'l',
        "--" => 'm',
        "-." => 'n',
        "---" => 'o',
        ".--." => 'p',
        "--.-" => 'q',
        ".-." => 'r',
        "..." => 's',
        "-" => 't',
        "..-" => 'u',
        "...-" => 'v',
        ".--" => 'w',
        "-..-" => 'x',
        "-.--" => 'y',
        "--.." => 'z',
        ".----" => '1',
        "..---" => '2',
        "...--" => '3',
        "....-" => '4',
        "....." => '5',
        "-...." => '6',
        "--..." => '7',
        "---.." => '8',
        "----." => '9',
        "-----" => '0',
        _ => unreachable!()
    }
}

pub fn morse_encrypt(text: &str) -> String {
    let mut out = String::new();
    for c in text.chars() {
        out.push_str(morse_in(c));
        out.push_str(" ");
    }
    out
}

pub fn morse_decrypt(text: &str) -> String {
    let mut out = String::new();
    for line in text.split_whitespace() {
        out.push(morse_out(line));
    }
    out
}

pub fn xor_manip(text: &str, key: u8) -> String {
    let mut out = Vec::with_capacity(text.len());
    for c in text.as_bytes() {
        out.push(c ^ key);
    }
    String::from_utf8(out).unwrap()
}

pub fn vigenere_encrypt(text: &str, key: &str) -> String {
    let mut out = Vec::with_capacity(text.len());
    let keys= key.as_bytes();

    for (i, c) in text.as_bytes().iter().enumerate() {
        let k = keys[i % keys.len()];
        let width = ((c - 'A' as u8) + (k - 'A' as u8)) % 26;
        out.push('A' as u8 + width);
    }
    String::from_utf8(out).unwrap()
}

pub fn vigenere_decrypt(text: &str, key: &str) -> String {
    let mut out = Vec::with_capacity(text.len());
    let keys= key.as_bytes();

    for (i, c) in text.as_bytes().iter().enumerate() {
        let k = keys[i % keys.len()];
        let width = ((c - 'A' as u8) + 26 - (k - 'A' as u8)) % 26;
        out.push('A' as u8 + width);
    }
    String::from_utf8(out).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_encode_sample01() {
        // 1st Test
        let original = "To err is human, but to really foul things up you need a computer.";
        let verify = "VG8gZXJyIGlzIGh1bWFuLCBidXQgdG8gcmVhbGx5IGZvdWwgdGhpbmdzIHVwIHlvdSBuZWVkIGEgY29tcHV0ZXIu";

        let base64_str = base64_encode(original);

        // verify encoding
        assert_eq!(base64_str, verify);

        let decode_str = base64_decode(&base64_str);
        // verify decoding
        assert_eq!(decode_str, original);
    }

    #[test]
    fn base64_encode_sample02() {
        // 2nd Test from [Wikipedia](https://en.wikipedia.org/wiki/Base64)
        let original =
            "Man is distinguished, not only by his reason, but by this singular \
            passion from other animals, which is a lust of the mind, that by a \
            perseverance of delight in the continued and indefatigable generation \
            of knowledge, exceeds the short vehemence of any carnal pleasure.";

        let verify =
            "TWFuIGlzIGRpc3Rpbmd1aXNoZWQsIG5vdCBvbmx5IGJ5IGhpcyByZWFzb24sIGJ1dCBieS\
            B0aGlzIHNpbmd1bGFyIHBhc3Npb24gZnJvbSBvdGhlciBhbmltYWxzLCB3aGljaCBpcyBh\
            IGx1c3Qgb2YgdGhlIG1pbmQsIHRoYXQgYnkgYSBwZXJzZXZlcmFuY2Ugb2YgZGVsaWdodC\
            BpbiB0aGUgY29udGludWVkIGFuZCBpbmRlZmF0aWdhYmxlIGdlbmVyYXRpb24gb2Yga25v\
            d2xlZGdlLCBleGNlZWRzIHRoZSBzaG9ydCB2ZWhlbWVuY2Ugb2YgYW55IGNhcm5hbCBwbG\
            Vhc3VyZS4=";

        let base64_str = base64_encode(original);

        // verify encoding
        assert_eq!(base64_str, verify);

        let decode_str = base64_decode(&base64_str);
        // verify decoding
        assert_eq!(decode_str, original);
    }

    #[test]
    fn caesar_sample() {
        let text1 = "ALANTURING";
        let en = caesar_encrypt(text1, 17);
        let de = caesar_decrypt(&en, 17);
        assert_eq!(text1, de);

        let text1 = "HELLOWORLD";
        let en = caesar_encrypt(text1, 1729);
        let de = caesar_decrypt(&en, 1729);
        assert_eq!(text1, de);
    }

    #[test]
    fn morse_sample() {
        let text = "0123456789";
        assert_eq!(&morse_decrypt(&morse_encrypt(text)), text);

        let text = "abcdefghijklmnopqrstuvwxyz";
        assert_eq!(&morse_decrypt(&morse_encrypt(text)), text);
    }

    #[test]
    fn xor_sample() {
        let text = "Whipalsh! : Do watch this movie...";
        assert_eq!(&xor_manip(&xor_manip(text, 17), 17), text);

        let text = "->Valar M0rghulis<-";
        assert_eq!(&xor_manip(&xor_manip(text, 29), 29), text);
    }

    #[test]
    fn vigenere_sample() {
        let text = "NIKOLATESLA"; let key = "TESLA";
        assert_eq!(&vigenere_decrypt(&vigenere_encrypt(text, key), key), text);

        let text = "GOOGLEIT"; let key = "REALLY";
        assert_eq!(&vigenere_decrypt(&vigenere_encrypt(text, key), key), text);
    }
}
