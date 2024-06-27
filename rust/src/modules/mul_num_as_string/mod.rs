pub fn multiply(a: &str, b: &str) -> String {
    let num1: Vec<u32> = a.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let num2: Vec<u32> = b.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let mut result = vec![0; num1.len() + num2.len()];

    for i in (0..num1.len()).rev() {
        for j in (0..num2.len()).rev() {
            let mul = num1[i] * num2[j];
            let sum = mul + result[i + j + 1];
            result[i + j] += sum / 10;
            result[i + j + 1] = sum % 10;
        }
    }

    while let Some(&0) = result.first() {
        result.remove(0);
    }

    let res = result
        .iter()
        .map(|&d| char::from_digit(d, 10).unwrap())
        .collect();

    if res == "" {
        "0".to_string()
    } else {
        res
    }
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
