pub fn next_smaller_number(n: u64) -> Option<u64> {
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
