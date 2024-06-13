// ISBN-10 Validation
// https://www.codewars.com/kata/51fc12de24a9d8cb0e000001
//
// Description:
//
// ISBN-10 identifiers are ten digits long. The first nine characters are digits 0-9. The last digit can be 0-9 or X, to indicate a value of 10.
//
// An ISBN-10 number is valid if the sum of the digits multiplied by their position modulo 11 equals zero.
//
// For example:
//
// ISBN     : 1 1 1 2 2 2 3 3 3  9
// position : 1 2 3 4 5 6 7 8 9 10
//
// This is a valid ISBN, because:
//
// (1*1 + 1*2 + 1*3 + 2*4 + 2*5 + 2*6 + 3*7 + 3*8 + 3*9 + 9*10) % 11 = 0
//
// Examples
//
// 1112223339   -->  true
// 111222333    -->  false
// 1112223339X  -->  false
// 1234554321   -->  true
// 1234512345   -->  false
// 048665088X   -->  true
// X123456788   -->  false
//

pub fn valid_isbn10(isbn: &str) -> bool {
    isbn.len().eq(&10)
        && isbn
            .chars()
            .enumerate()
            .try_fold(0, |sum, (i, ch)| {
                let num = if ch.is_ascii_digit() {
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
