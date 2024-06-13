pub fn solution(num: i32) -> i32 {
    (1..num).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

pub fn _solution(num: i32) -> i32 {
    let mut sum = 0;

    let max_3 = num / 3 + std::cmp::min(num % 3, 1);
    let max_5 = num / 5 + std::cmp::min(num % 5, 1);

    for i in 1..num {
        if i < max_3 {
            sum += i * 3;
        };
        if i < max_5 && i % 3 != 0 {
            sum += i * 5;
        };
    }

    sum
}

mod tests {
    use super::solution;

    #[test]
    fn sample_tests() {
        // assertion(expected, input);
        assertion(23, 10);
        assertion(33, 11);
        assertion(225, 33);
        assertion(8, 6);
        assertion(3420, 123);
        assertion(543, 50);
        assertion(0, 0);
        assertion(0, -203);
        assertion(25719750, 10500);
    }

    #[allow(dead_code)]
    fn assertion(expected: i32, input: i32) {
        let actual = solution(input);

        assert!(
            expected == actual,
            "\nTest failed!\n expected: {}\n actual: {}\n input: {}\n",
            expected,
            actual,
            input
        );
    }
}
