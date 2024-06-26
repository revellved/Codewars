pub fn multiply(a: &str, b: &str) -> String {
    if a.chars().all(|c| c == '0') || b.chars().all(|c| c == '0') {
        return String::from('0');
    }

    let mut res = vec![0; a.len() + b.len()];

    for (i, c1) in a.chars().rev().enumerate() {
        for (j, c2) in b.chars().rev().enumerate() {
            let n = (c1 as u8 - 48) * (c2 as u8 - 48) + res[i + j];
            res[i + j] = n % 10;
            res[i + j + 1] += n / 10;
        }
    }

    res.iter()
        .rev()
        .skip_while(|&&i| i == 0)
        .map(|i| (i + 48) as char)
        .collect()
}

#[cfg(test)]
mod sample_tests {
    use super::multiply;

    fn do_test(a: &str, b: &str, expected: &str) {
        let actual = multiply(&a, &b);
        assert_eq!(actual, expected,
               "\n\nMultiplying a*b with\na = {a}\nb = {b}\nshould return: {expected}\ninstead got: {actual}");
    }

    #[test]
    fn simple_cases() {
        //        input       expected
        do_test("2", "3", "6");
        do_test("30", "69", "2070");
        do_test("11", "85", "935");
    }

    #[test]
    fn edge_cases() {
        do_test("2", "0", "0");
        do_test("0", "30", "0");
        do_test("0000001", "3", "3");
        do_test("1009", "03", "3027");
    }

    #[test]
    fn big_numbers() {
        do_test("98765", "56894", "5619135910");
        do_test(
            "9007199254740991",
            "9007199254740991",
            "81129638414606663681390495662081",
        );
        do_test(
            "1020303004875647366210",
            "2774537626200857473632627613",
            "2830869077153280552556547081187254342445169156730",
        );
        do_test(
            "58608473622772837728372827",
            "7586374672263726736374",
            "444625839871840560024489175424316205566214109298",
        );
    }
}
