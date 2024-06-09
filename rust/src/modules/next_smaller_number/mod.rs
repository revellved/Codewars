use itertools::Itertools;
// use num_traits::PrimInt;

pub fn next_smaller_number(n: u64) -> Option<u64> {
    // println!("n: {n}, n if i=2: {:?}", num_get_index(n, 1));
    // let mut div: u64 = 10;
    // while n > div {
    //     let target_num = n % div - (n % (div / 10));
    //
    //     div *= 10
    // }

    let nn: u64 = n
        .to_string()
        .replace("0", "@")
        .chars()
        .sorted()
        .join("")
        .replace("@", "0")
        .parse()
        .unwrap();
    match nn == n {
        true => None,
        false => Some(nn),
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
