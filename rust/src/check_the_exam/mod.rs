// **PROLOG REFERENCE**
// maplist([X,Y,R]>>(Y='' -> R is 0; X=Y -> R is 4; R is -1), Array1, Array2, Scores),
// sumlist(Scores, Sum),
// Score is max(0, Sum).

pub fn check_exam(arr_a: &[&str], arr_b: &[&str]) -> i64 {
    arr_b
        .into_iter()
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
