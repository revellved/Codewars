pub fn persistence(mut num: u64) -> u64 {
    let mut iterations_count = 0;
    while num % 10 != num {
        let mut div = 10;
        let mut sum = 1;
        while div / 10 < num {
            sum *= num % div / (div / 10);
            div *= 10;
        }
        num = sum;
        iterations_count += 1;
    }

    iterations_count
}

#[cfg(test)]
mod tests {
    use super::persistence;

    #[test]
    fn sample_tests() {
        assert_eq!(persistence(39), 3);
        assert_eq!(persistence(4), 0);
        assert_eq!(persistence(25), 2);
        assert_eq!(persistence(999), 4);
    }
}
