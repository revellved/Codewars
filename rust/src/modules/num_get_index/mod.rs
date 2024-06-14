use num_traits::PrimInt;
//
// pub fn num_get_index<N: PrimInt>(n: N, i: u32) -> Option<N> {
//     let div = 10u32.pow(i - 1);
//     let div_right: N = N::from(div).unwrap();
//     let div_left: N = N::from(div * 10).unwrap();
//     match div_right > n {
//         true => None,
//         false => Some((n % div_left - (n % div_right)) / div_right),
//     }
// }
pub fn num_get_index<N: PrimInt>(n: N, i: u32) -> Option<N> {
    let div = 10u32.pow(i - 1);
    let div_right: N = N::from(div).unwrap();
    let div_left: N = N::from(div * 10).unwrap();
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
