mod preloaded;

use itertools::Itertools;
// use encoding::all::ASCII;
// use itertools::Itertools;
pub use preloaded::MORSE_CODE;

// use itertools::Itertools;
// use crate::decode_morse;
// MORSE_CODE is `HashMap<String, String>`. e.g. ".-" -> "A".

fn decode_bit(encoded: &str) -> u8 {
    encoded.chars().rev().enumerate().fold(0, |num, (i, ch)| {
        if ch == '1' {
            num + u32::pow(2, i as u32)
        } else {
            num
        }
    }) as u8
    // match encoded {
    //     "11001100" => '.'.
    //     ""
    //     _ => ' '
    // } as u8
}

pub fn decode_bits(encoded: &str) -> String {
    let mut i = 0;
    let mut s: Vec<u8> = vec![];
    while i + 8 < encoded.len() {
        let bit = decode_bit(&encoded[i..(i + 8)]);
        s.push(bit);

        i += 8;
    }

    String::from_utf8(s).unwrap_or(String::new())
}

pub fn _decode_morse(encoded: &str) -> String {
    encoded
        .replace("  ", " @ ")
        .split_whitespace()
        .fold(String::new(), |a, s| match s {
            "" => a,
            "@" if a.chars().last().unwrap_or(' ').ne(&' ') => a + " ",
            _ => a + (*MORSE_CODE).get(s).unwrap_or(&String::new()),
        })
}

pub fn __decode_morse(encoded: &str) -> String {
    encoded
        .split("  ")
        .filter_map(|word| match word {
            "" => None,
            _ => Some(word.split_whitespace().fold(String::new(), |a, s| {
                a + (*MORSE_CODE).get(s).unwrap_or(&String::new())
            })),
        })
        .join(" ")
}

pub fn ___decode_morse(encoded: &str) -> String {
    encoded
        .split("  ")
        .fold(String::new(), |final_string, word| {
            (match final_string.len() {
                0 => final_string,
                _ => final_string + " ",
            }) + word
                .split_whitespace()
                .fold(String::new(), |s, ch| {
                    s + (*MORSE_CODE).get(ch).unwrap_or(&String::new())
                })
                .as_str()
        })
}

pub fn decode_morse(encoded: &str) -> String {
    encoded
        .split(' ')
        .map(|x| MORSE_CODE.get(x).unwrap_or(&" ".to_string()).clone())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn examples() {
    assert_eq!(
        decode_morse("  .... . -.--              .--- ..- -.. ."),
        "HEY JUDE".to_string()
    );
}

// #[test]
// fn examples2() {
//     assert_eq!(decode_morse(&decode_bits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011")), "HEY JUDE".to_string());
// }
