// Replace With Alphabet Position
// https://www.codewars.com/kata/546f922b54af40e1e90001da
//
// Description:
//
// Welcome.
//
// In this kata you are required to, given a string, replace every letter with its position in the alphabet.
//
// If anything in the text isn't a letter, ignore it and don't return it.
//
// "a" = 1, "b" = 2, etc.
// Example
//
// Input = "The sunset sets at twelve o' clock."
// Output = "20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11"
//

use std::char;

fn _alphabet_position(text: &str) -> String {
    let mut res = String::from("");
    text.to_lowercase().chars().for_each(|c: char| {
        let v = c as i64 - 96;
        if v > 0 && v < 27 {
            if !res.is_empty() {
                res += " "
            }
            res += &(v).to_string();
        }
    });

    res
}

pub fn alphabet_position(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter(|c| (&'a'..=&'z').contains(&c))
        .map(|c| (c as u32 - 96).to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(
            alphabet_position("The sunset sets at twelve o' clock."),
            "20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11".to_string()
        );
        assert_eq!(
            alphabet_position("The narwhal bacons at midnight."),
            "20 8 5 14 1 18 23 8 1 12 2 1 3 15 14 19 1 20 13 9 4 14 9 7 8 20".to_string()
        );
    }
}
