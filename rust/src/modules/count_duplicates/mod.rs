// Counting Duplicates
// https://www.codewars.com/kata/54bf1c2cd5b56cc47f0007a1
//
// Description:
// Count the number of Duplicates
//
// Write a function that will return the count of distinct case-insensitive alphabetic characters and numeric digits that occur more than once in the input string. The input string can be assumed to contain only alphabets (both uppercase and lowercase) and numeric digits.
// Example
//
// "abcde" -> 0 # no characters repeats more than once
// "aabbcde" -> 2 # 'a' and 'b'
// "aabBcde" -> 2 # 'a' occurs twice and 'b' twice (`b` and `B`)
// "indivisibility" -> 1 # 'i' occurs six times
// "Indivisibilities" -> 2 # 'i' occurs seven times and 's' occurs twice
// "aA11" -> 2 # 'a' and '1'
// "ABBA" -> 2 # 'A' and 'B' each occur twice
//

use itertools::Itertools;
use std::collections::HashMap;

pub fn _count_duplicates(text: &str) -> u32 {
    let mut used_chars: HashMap<char, u32> = HashMap::new();
    text.to_lowercase().chars().fold(0, |count, c: char| {
        let i = used_chars.entry(c).or_default();
        *i += 1;
        if *i == 2 {
            count + 1
        } else {
            count
        }
    })
}

pub fn count_duplicates(text: &str) -> u32 {
    text.to_lowercase()
        .chars()
        .counts()
        .values()
        .filter(|&&i| i > 1)
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abcde() {
        assert_eq!(count_duplicates("abcde"), 0);
    }

    #[test]
    fn test_abcdea() {
        assert_eq!(count_duplicates("abcdea"), 1);
    }

    #[test]
    fn test_indivisibility() {
        assert_eq!(count_duplicates("indivisibility"), 1);
    }
}
