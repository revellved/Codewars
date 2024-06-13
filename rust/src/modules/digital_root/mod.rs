// Sum of Digits / Digital Root
// https://www.codewars.com/kata/541c8630095125aba6000c00
//
// Description:
//
// Digital root is the recursive sum of all the digits in a number.
//
// Given n, take the sum of the digits of n. If that value has more than one digit, continue reducing in this way until a single-digit number is produced. The input will be a non-negative integer.
// Examples
//
//     16  -->  1 + 6 = 7
//    942  -->  9 + 4 + 2 = 15  -->  1 + 5 = 6
// 132189  -->  1 + 3 + 2 + 1 + 8 + 9 = 24  -->  2 + 4 = 6
// 493193  -->  4 + 9 + 3 + 1 + 9 + 3 = 29  -->  2 + 9 = 11  -->  1 + 1 = 2
//

fn _1_digital_root(n: i64) -> i64 {
    let str_num = n.to_string();
    let mut sum_nums: i64 = 0;

    for i in str_num.chars() {
        sum_nums += i as i64 - 48;
    }

    sum_nums
}

fn _sum_ordire_enumarate(n: i64) -> i64 {
    n.to_string().chars().fold(0, |mut sum, c| {
        sum += c as i64 - 48;
        sum
    })
}

fn _2_digital_root(mut n: i64) -> i64 {
    while n >= 10 {
        n = _sum_ordire_enumarate(n)
    }
    n
}

fn _3_digital_root(mut n: i64) -> i64 {
    while n >= 10 {
        n = n.to_string().chars().fold(0, |mut sum, c| {
            sum += c as i64 - 48;
            sum
        })
    }
    n
}

pub fn digital_root(n: i64) -> i64 {
    (n - 1) % 9 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(digital_root(16), 7);
        assert_eq!(digital_root(942), 6);
    }
}
