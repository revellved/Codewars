// Sum of Pairs
// https://www.codewars.com/kata/54d81488b981293527000c8f
//
// Description:
// Sum of Pairs
//
// Given a list of integers and a single sum value, return the first two values (parse from the left please) in order of appearance that add up to form the sum.
//
// If there are two or more pairs with the required sum, the pair whose second element has the smallest index is the solution.
//
// sum_pairs([11, 3, 7, 5],         10)
// #              ^--^      3 + 7 = 10
// == [3, 7]
//
// sum_pairs([4, 3, 2, 3, 4],         6)
// #          ^-----^         4 + 2 = 6, indices: 0, 2 *
// #             ^-----^      3 + 3 = 6, indices: 1, 3
// #                ^-----^   2 + 4 = 6, indices: 2, 4
// #  * the correct answer is the pair whose second value has the smallest index
// == [4, 2]
//
// sum_pairs([0, 0, -2, 3], 2)
// #  there are no pairs of values that can be added to produce 2.
// == None/nil/undefined (Based on the language)
//
// sum_pairs([10, 5, 2, 3, 7, 5],         10)
// #              ^-----------^   5 + 5 = 10, indices: 1, 5
// #                    ^--^      3 + 7 = 10, indices: 3, 4 *
// #  * the correct answer is the pair whose second value has the smallest index
// == [3, 7]
//
// Negative numbers and duplicate numbers can and will appear.
//
// NOTE: There will also be lists tested of lengths upwards of 10,000,000 elements. Be sure your code doesn't time out.
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
