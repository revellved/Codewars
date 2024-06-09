mod solution {
    use std::cmp;

    #[allow(dead_code)]
    pub fn range_extraction(a: &[i32]) -> String {
        let mut last = 0;
        let mut range: Option<cmp::Ordering> = None;
        a.iter().fold(String::new(), |s, n| {
            if s.len() == 0 {
                last = *n;
                return n.to_string();
            }

            // println!(
            //     "last: {}, n: {}, last-n: {}  |  str: ({s})",
            //     last,
            //     *n,
            //     last - *n
            // );

            if (last - *n).abs() == 1 {
                last = *n;
                range = Some(last.cmp(n));
                return s;
            };

            if range.is_some() {
                range = None;
                s + "-" + last.to_string().as_str() + "," + n.to_string().as_str()
            } else {
                s + "," + n.to_string().as_str()
            }
        })
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example() {
        // assert_eq!(
        //     solution::range_extraction(&[
        //         -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20
        //     ]),
        //     "-6,-3-1,3-5,7-11,14,15,17-20"
        // );
        // assert_eq!(
        //     solution::range_extraction(&[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20]),
        //     "-3--1,2,10,15,16,18-20"
        // );
    }
}
