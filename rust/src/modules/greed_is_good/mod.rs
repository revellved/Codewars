pub fn score(dice: [u8; 5]) -> u32 {
    let mut score = 0;
    let mut counts = [0; 6];

    for &die in dice.iter() {
        counts[(die - 1) as usize] += 1;
    }

    for (i, &count) in counts.iter().enumerate() {
        match i {
            0 => {
                if count >= 3 {
                    score += 1000 + (count - 3) * 100;
                } else {
                    score += count * 100;
                }
            }
            4 => {
                if count >= 3 {
                    score += 500 + (count - 3) * 50;
                } else {
                    score += count * 50;
                }
            }
            _ => {
                if count >= 3 {
                    score += (i as u32 + 1) * 100;
                }
            }
        }
    }

    score
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
