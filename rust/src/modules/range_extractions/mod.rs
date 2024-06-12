// https://www.codewars.com/kata/51ba717bb08c1cd60f00002f
//
// Description:
//
// A format for expressing an ordered list of integers is to use a comma separated list of either
//
//     individual integers
//     or a range of integers denoted by the starting integer separated from the end integer in the range by a dash, '-'. The range includes all integers in the interval including both endpoints. It is not considered a range unless it spans at least 3 numbers. For example "12,13,15-17"
//
// Complete the solution so that it takes a list of integers in increasing order and returns a correctly formatted string in the range format.
//
// Example:
//
// solution([-10, -9, -8, -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20]);
// // returns "-10--8,-6,-3-1,3-5,7-11,14,15,17-20"
//
// Courtesy of rosettacode.org
//

pub fn range_extraction(a: &[i32]) -> String {
    let mut result = String::new();
    let mut i = 0;

    while i < a.len() {
        let start = a[i];
        let mut end = start;

        while i < a.len() - 1 && a[i + 1] == end + 1 {
            end = a[i + 1];
            i += 1;
        }

        if start == end {
            result.push_str(&format!("{},", start));
        } else if end == start + 1 {
            result.push_str(&format!("{},{},", start, end));
        } else {
            result.push_str(&format!("{}-{},", start, end));
        }

        i += 1;
    }

    result.pop(); // Remove the trailing comma
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            range_extraction(&[
                -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20
            ]),
            "-6,-3-1,3-5,7-11,14,15,17-20"
        );
        assert_eq!(
            range_extraction(&[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20]),
            "-3--1,2,10,15,16,18-20"
        );
    }
}
