mod preloaded;

pub use preloaded::MORSE_CODE;

fn decode_bit(encoded: &str) -> u8 {
    encoded.chars().rev().enumerate().fold(0, |num, (i, ch)| {
        if ch == '1' {
            num + u32::pow(2, i as u32)
        } else {
            num
        }
    }) as u8
}

pub fn decode_bits(encoded: &str) -> String {
    let mut i = 0;
    let mut s: Vec<u8> = vec![];
    while i + 8 < encoded.len() {
        let bit = decode_bit(&encoded[i..(i + 8)]);
        s.push(bit);

        i += 8;
    }

    String::from_utf8(s).unwrap_or_default()
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
