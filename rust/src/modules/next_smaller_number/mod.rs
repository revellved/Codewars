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
