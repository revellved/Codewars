// Persistent Bugger.
// https://www.codewars.com/kata/55bf01e5a717a0d57e0000ec
//
// Description:
//
// Write a function, persistence, that takes in a positive parameter num and returns its multiplicative persistence, which is the number of times you must multiply the digits in num until you reach a single digit.
//
// For example (Input --> Output):
//
// 39 --> 3 (because 3*9 = 27, 2*7 = 14, 1*4 = 4 and 4 has only one digit, there are 3 multiplications)
// 999 --> 4 (because 9*9*9 = 729, 7*2*9 = 126, 1*2*6 = 12, and finally 1*2 = 2, there are 4 multiplications)
// 4 --> 0 (because 4 is already a one-digit number, there is no multiplication)
//

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
