use itertools::Itertools;
use std::collections::HashMap;

pub fn order(s: &str) -> String {
    s.split_whitespace()
        .sorted_by_key(|word| word.chars().find_map(|c| c.to_digit(10)))
        .join(" ")
}

fn _order(text: &str) -> String {
    let mut order_map: HashMap<i32, &str> = HashMap::new();
    let mut index: i32 = 0;
    let words = text.split(" ");
    let count_words = words.clone().count() as i32;

    words.for_each(|word| {
        let mut key = word.chars().fold(0, |i: i32, char_of_word| {
            if char_of_word.is_ascii_digit() {
                i + char_of_word as i32 - 48
            } else {
                i
            }
        });
        key = if key == 0 {
            index -= 1;
            index
        } else {
            key
        };
        *order_map.entry(key).or_default() = word;
    });

    let mut i = 1;
    let mut j = -1;
    let mut result: Vec<&str> = vec![];

    while i <= count_words {
        let mut v = *order_map.entry(i).or_default();
        if v.is_empty() {
            v = *order_map.entry(j).or_default();
            j -= 1;
        }
        result.push(v);
        i += 1;
    }

    result.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easy() {
        assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
    }

    #[test]
    fn test_middle() {
        assert_eq!(
            order("4of Fo1r pe6ople g3ood th5e the2"),
            "Fo1r the2 g3ood 4of th5e pe6ople"
        );
    }
}
