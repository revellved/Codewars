pub fn count_ones(left: u64, right: u64) -> u64 {
    (left..=right).fold(0, |mut count, mut i| {
        while i > 0 {
            count += i % 2;
            i /= 2;
        }
        count
    }) as u64
}

#[cfg(test)]
mod tests {
    use super::count_ones;

    #[test]
    fn sample_tests() {
        assert_eq!(count_ones(5, 7), 7);
        assert_eq!(count_ones(12, 29), 51);
    }
}
