pub fn check_exam(arr_a: &[&str], arr_b: &[&str]) -> i64 {
    arr_b
        .iter()
        .enumerate()
        .fold(0, |acc, (i, v)| match *v {
            "" => acc,
            a if (a == arr_a[i]) => acc + 4,
            _ => acc - 1,
        })
        .max(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(check_exam(&["a", "a", "b", "b"], &["a", "c", "b", "d"]), 6);
        assert_eq!(check_exam(&["a", "a", "c", "b"], &["a", "a", "b", ""]), 7);
        assert_eq!(check_exam(&["a", "a", "b", "c"], &["a", "a", "b", "c"]), 16);
        assert_eq!(check_exam(&["b", "c", "b", "a"], &["", "a", "a", "c"]), 0);
    }
}
