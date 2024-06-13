// Number of trailing zeros of N!
// https://www.codewars.com/kata/52f787eb172a8b4ae1000a34
//
// Description:

// Write a program that will calculate the number of trailing zeros in a factorial of a given number.
//
// N! = 1 * 2 * 3 *  ... * N
//
// Be careful 1000! has 2568 digits...
//
// For more info, see: http://mathworld.wolfram.com/Factorial.html
// Examples
// N 	Product 	N factorial 	Trailing zeros
// 6 	1*2*3*4*5*6 	720 	1
// 12 	1*2*3*4*5*6*7*8*9*10*11*12 	479001600 	2
//
// Hint: You're not meant to calculate the factorial. Find another way to find the number of zeros.
//

pub fn zeros(n: u64) -> u64 {
    let mut res = 0;
    let mut div = 5;
    while div <= n {
        res += n / div;
        div *= 5;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(zeros(0), 0);
        assert_eq!(zeros(6), 1);
        assert_eq!(zeros(14), 2);
        assert_eq!(zeros(30), 7);
        assert_eq!(zeros(1000), 249);
        assert_eq!(zeros(100000), 24999);
        assert_eq!(zeros(1000000000), 249999998);
    }
}
