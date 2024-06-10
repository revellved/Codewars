pub fn valid_isbn10(isbn: &str) -> bool {
    isbn.len().eq(&10)
        && isbn
            .chars()
            .enumerate()
            .try_fold(0, |sum, (i, ch)| {
                let num = if ('0'..='9').contains(&ch) {
                    ch as usize - 48
                } else if i == 9 && ch == 'X' {
                    10
                } else {
                    return None;
                };
                Some(sum + (i + 1) * num)
            })
            .unwrap_or(1)
            % 11
            == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(isbn: &str, expected: bool) {
        let actual = valid_isbn10(isbn);
        assert!(
            actual == expected,
            "Test failed with isbn = {isbn}\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn sample_tests() {
        dotest("1112223339", true);
        dotest("048665088X", true);
        dotest("1293000000", true);
        dotest("1234554321", true);
        dotest("1234512345", false);
        dotest("1293", false);
        dotest("X123456788", false);
        dotest("ABCDEFGHIJ", false);
        dotest("XXXXXXXXXX", false);
        dotest("123456789T", false);
        dotest("5758144266", true);
        dotest("0969251496", false);
    }
}
