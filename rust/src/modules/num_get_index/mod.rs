pub fn num_get_index(n: u32, i: u32) -> Option<u32> {
    let div = 10u32.pow(i - 1);
    let div_right = div;
    let div_left = div * 10;
    match div_right > n {
        true => None,
        false => Some((n % div_left - (n % div_right)) / div_right),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        assert_eq!(num_get_index(123, 1), Some(3));
        assert_eq!(num_get_index(123, 2), Some(2));
        assert_eq!(num_get_index(123, 3), Some(1));
        assert_eq!(num_get_index(123, 4), None);
    }
}
