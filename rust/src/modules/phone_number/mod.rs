// Create Phone Number
// https://www.codewars.com/kata/525f50e3b73515a6db000b83
//
// Description:
//
// Write a function that accepts an array of 10 integers (between 0 and 9), that returns a string of those numbers in the form of a phone number.
// Example
//
// create_phone_number(&[1,2,3,4,5,6,7,8,9,0]); // returns "(123) 456-7890"
//
// The returned format must be correct in order to complete this challenge.
//
// Don't forget the space after the closing parentheses!
//

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
