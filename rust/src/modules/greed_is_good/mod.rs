fn score(dice: [u8; 5]) -> u32 {
    let mut numbers = [0u32; 6];
    for n in dice {
        numbers[(n - 1) as usize] += 1;
    }
    numbers[0] / 3 * 1000 
    + numbers[0] % 3 * 100
    + numbers[1] / 3 * 200
    + numbers[2] / 3 * 300
    + numbers[3] / 3 * 400
    + numbers[4] / 3 * 500
    + numbers[4] % 3 * 50
    + numbers[5] / 3 * 600
}

#[cfg(test)]
mod tests {
    use super::score;

    fn dotest(dice: [u8; 5], expected: u32) {
        let actual = score(dice);
        assert!(
            actual == expected,
            "Expected score with dice {dice:?} to be {expected}, but was {actual}\n"
        );
    }

    #[test]
    fn sample_tests() {
        dotest([2, 3, 4, 6, 2], 0);
        dotest([4, 4, 4, 3, 3], 400);
        dotest([2, 4, 4, 5, 4], 450);
    }
}
