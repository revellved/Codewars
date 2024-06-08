use itertools::Itertools;
use std::collections::HashMap;

fn nums_neighbors(ch: char) -> Vec<String> {
    match ch {
        '0' => "08",
        '1' => "124",
        '2' => "1235",
        '3' => "236",
        '4' => "1457",
        '5' => "24568",
        '6' => "3569",
        '7' => "478",
        '8' => "05789",
        '9' => "689",
        _ => "",
    }
    .chars()
    .map(|s| s.to_string())
    .collect_vec()
}

pub fn get_pins_old(observed: &str) -> Vec<String> {
    observed
        .chars()
        .into_iter()
        .fold(vec![String::new()], |pins, num| {
            pins.into_iter()
                .map(|pin| {
                    nums_neighbors(num)
                        .iter()
                        .map(|n| pin.clone() + n)
                        .collect_vec()
                })
                .collect_vec()
                .concat()
        })
}

pub fn get_pins(observed: &str) -> Vec<String> {
    let possibilities = HashMap::from([
        ('0', vec!['0', '8']),
        ('1', vec!['1', '2', '4']),
        ('2', vec!['1', '2', '3', '5']),
        ('3', vec!['2', '3', '6']),
        ('4', vec!['1', '4', '5', '7']),
        ('5', vec!['2', '4', '5', '6', '8']),
        ('6', vec!['3', '5', '6', '9']),
        ('7', vec!['4', '7', '8']),
        ('8', vec!['0', '5', '7', '8', '9']),
        ('9', vec!['6', '8', '9']),
    ]);

    observed
        .chars()
        .map(|c| possibilities.get(&c).unwrap())
        .multi_cartesian_product()
        .map(|v| v.into_iter().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::get_pins;
    use itertools::Itertools;

    #[test]
    fn sample_tests() {
        assert_eq!(
            get_pins("8").iter().sorted().collect::<Vec<&String>>(),
            vec!["0", "5", "7", "8", "9"]
        );
        assert_eq!(
            get_pins("11").iter().sorted().collect::<Vec<&String>>(),
            vec!["11", "12", "14", "21", "22", "24", "41", "42", "44"]
        );
        assert_eq!(
            get_pins("369").iter().sorted().collect::<Vec<&String>>(),
            vec![
                "236", "238", "239", "256", "258", "259", "266", "268", "269", "296", "298", "299",
                "336", "338", "339", "356", "358", "359", "366", "368", "369", "396", "398", "399",
                "636", "638", "639", "656", "658", "659", "666", "668", "669", "696", "698", "699"
            ]
        );
    }
}
