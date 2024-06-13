// Next smaller number with the same digits
// https://www.codewars.com/kata/5659c6d896bc135c4c00021e/rust
//
// Description:
//
// Write a function that takes a positive integer and returns the next smaller positive integer containing the same digits.
//
// For example:
//
// next_smaller(21) == Some(12)
// next_smaller(531) == Some(513)
// next_smaller(2071) == Some(2017)
//
// Return -1 (for Haskell: return Nothing, for Rust: return None), when there is no smaller number that contains the same digits. Also return -1 when the next smaller number with the same digits would require the leading digit to be zero.
//
// next_smaller(9) == None
// next_smaller(135) == None
// next_smaller(1027) == None  // 0721 is out since we don't write numbers with leading zeros
//
//     some tests will include very large numbers.
//     test data only employs positive integers.
//
// The function you write for this challenge is the inverse of this kata: "Next bigger number with the same digits."
//

pub fn next_smaller_number(n: u64) -> Option<u64> {
    let mut digs = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    let bnp = digs.windows(2).rposition(|d| d[0] > d[1])?;
    let bn = digs[bnp];
    let mn = digs[bnp + 1..].iter().filter(|&d| d < &bn).max()?;
    let mnp = digs.iter().rposition(|&d| d == *mn)?;
    digs.swap(bnp, mnp);
    let (_, tail) = digs.split_at_mut(bnp + 1);
    tail.reverse();
    if digs[0] == 0 {
        return None;
    }
    digs.into_iter()
        .map(|d| std::char::from_digit(d, 10).unwrap())
        .collect::<String>()
        .parse::<u64>()
        .ok()
}

pub fn _next_smaller_number(n: u64) -> Option<u64> {
    let mut digits: Vec<u64> = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();

    let mut i = digits.len() as i32 - 1;
    while i > 0 && digits[i as usize] >= digits[(i - 1) as usize] {
        i -= 1;
    }

    if i == 0 {
        return None;
    }

    let mut j = digits.len() as i32 - 1;
    while j > 0 && digits[j as usize] >= digits[(i - 1) as usize] {
        j -= 1;
    }

    digits.swap((i - 1) as usize, j as usize);

    digits[(i as usize)..].reverse();

    let result: u64 = digits.iter().fold(0, |acc, &x| acc * 10 + x);

    if result.to_string().len() == digits.len() {
        Some(result)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(next_smaller_number(21), Some(12));
        // assert_eq!(next_smaller_number(907), Some(790));
        // assert_eq!(next_smaller_number(531), Some(513));
        // assert_eq!(next_smaller_number(441), Some(414));
        // assert_eq!(next_smaller_number(1027), None); // 0721 is out since we don't write numbers with leading zeros
        // assert_eq!(next_smaller_number(9), None);
        // assert_eq!(next_smaller_number(135), None);
        // assert_eq!(next_smaller_number(1027), None);
    }
}
