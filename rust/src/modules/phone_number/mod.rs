pub fn create_phone_number(numbers: &[u8]) -> String {
    let s: String = numbers.iter().map(|i| i.to_string()).collect();
    format!("({}) {}-{}", &s[..3], &s[3..6], &s[6..])
}

pub fn __create_phone_number(numbers: &[u8]) -> String {
    let mut res_str: String = String::from("(");
    for dig in &numbers[0..3] {
        res_str += dig.to_string().as_str();
    }
    res_str += ") ";
    for dig in &numbers[3..6] {
        res_str += dig.to_string().as_str();
    }
    res_str += "-";
    for dig in &numbers[6..10] {
        res_str += dig.to_string().as_str();
    }

    res_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(
            create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
            "(123) 456-7890"
        );
        assert_eq!(
            create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            "(111) 111-1111"
        );
        assert_eq!(
            create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]),
            "(123) 456-7899"
        );
    }
}
