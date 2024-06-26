pub fn total_inc_dec(n: u32) -> u64 {
    match n < 3 {
        true => 10u64.pow(n),
        false => {
            let n = n as u64;
            let r = n + 9;
            let max = if n <= r / 2 { n } else { 9 };
            let n_cr = (0..max).fold(1u64, |acc, i| acc * (r - i) / (i + 1));
            (20 + n) * n_cr / 10 - 10 * n - 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::total_inc_dec;

    fn dotest(n: u32, expected: u64) {
        let actual = total_inc_dec(n);
        assert!(
            actual == expected,
            "With n = {n}\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(0, 1);
        dotest(1, 10);
        dotest(2, 100);
        dotest(3, 475);
        dotest(4, 1675);
    }
}
