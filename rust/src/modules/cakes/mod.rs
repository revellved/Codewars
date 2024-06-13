// Pete, the baker
// https://www.codewars.com/kata/525c65e51bf619685c000059
//
// Description:
//
// Pete likes to bake some cakes. He has some recipes and ingredients. Unfortunately he is not good in maths. Can you help him to find out, how many cakes he could bake considering his recipes?
//
// Write a function cakes(), which takes the recipe (HashMap<&str, u32>) and the available ingredients (also a HashMap<&str, u32>) and returns the maximum number of cakes Pete can bake (u32). For simplicity there are no units for the amounts (e.g. 1 lb of flour or 200 g of sugar are simply 1 or 200). Ingredients that are not present in the objects, can be considered as 0.
//
// Examples:
//
// // must return 2
// cakes(HashMap::from([("flour", 500), ("sugar", 200), ("eggs", 1)]), HashMap::from([("flour", 1200), ("sugar", 1200), ("eggs", 5), ("milk", 200)]))
// // must return 0
// cakes(HashMap::from([("apples", 3), ("flour", 300), ("sugar", 150), ("milk", 100), ("oil", 100)]), HashMap::from([("sugar", 500),("flour", 2000), ("milk", 2000)]))
//

use std::collections::HashMap;

pub fn cakes(recipe: &HashMap<&str, u32>, available: &HashMap<&str, u32>) -> u32 {
    recipe.iter().fold(u32::MAX, |count, product| {
        let available_product = available.get(product.0);
        if let Some(ap) = available_product {
            std::cmp::min(count, ap / product.1)
        } else {
            0
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    macro_rules! map {
        () => { HashMap::new() };
        ($($ingredient:ident : $amount:expr),+) => {{
            let mut map = HashMap::new();
            $( map.insert(stringify!($ingredient), $amount); )*
            map
        }};
    }

    fn test(recipe: &HashMap<&str, u32>, available: &HashMap<&str, u32>, expected: u32) {
        let actual = cakes(recipe, available);
        assert!(actual == expected, "Expected to bake {expected} cakes, but got {actual} cakes instead.\nAvailable ingredients:\n  {available:?}\nRecipe:\n  {recipe:?}\n\n");
    }

    #[test]
    fn test_example() {
        let recipe = map!(flour: 500, sugar: 200, eggs: 1);
        let available = map!(flour: 1200, sugar: 1200, eggs: 5, milk: 200);
        test(&recipe, &available, 2);

        let recipe = map!(apples: 3, flour: 300, sugar: 150, milk: 100, oil: 100);
        let available = map!(sugar: 500, flour: 2000, milk: 2000);
        test(&recipe, &available, 0);
    }
}
