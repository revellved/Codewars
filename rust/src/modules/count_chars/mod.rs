use std::collections::HashMap;

pub fn count_chars(input: &str) -> HashMap<char, i32> {
    let mut used_chars: HashMap<char, i32> = HashMap::new();
    input.to_lowercase().chars().for_each(|c: char| {
        *used_chars.entry(c).or_default() += 1;
    });
    used_chars
}

#[cfg(test)]
mod tests {
    use super::*;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    #[test]
    fn test_empty_string() {
        let test_input = "";
        let expected: HashMap<char, i32> = HashMap::new();

        assert_eq!(
            count_chars(test_input),
            expected,
            "{ERR_MSG} with input: \"{test_input}\""
        );
    }

    #[test]
    fn test_string_with_two_equal_letters() {
        let test_input = "aa";
        let mut expected: HashMap<char, i32> = HashMap::new();
        expected.insert('a', 2);

        assert_eq!(
            count_chars(test_input),
            expected,
            "{ERR_MSG} with input: \"{test_input}\""
        );
    }

    #[test]
    fn test_string_with_different_letters() {
        let test_input = "aabb";
        let mut expected: HashMap<char, i32> = HashMap::new();
        expected.insert('a', 2);
        expected.insert('b', 2);

        assert_eq!(
            count_chars(test_input),
            expected,
            "{ERR_MSG} with input: \"{test_input}\""
        );
    }
}
