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
