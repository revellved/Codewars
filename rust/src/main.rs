use alphabet_position::alphabet_position;
use codewars::map_key_val;
use count_chars::count_chars;
use count_duplicates::count_duplicates;
use digital_root::digital_root;
use hello::hello;
use unique_in_order::unique_in_order;

mod alphabet_position;
mod count_chars;
mod count_duplicates;
mod digital_root;
mod hello;
mod unique_in_order;

fn main() {
    let text = hello().clone();
    println!("{text}");

    assert_eq!(
        alphabet_position(text.as_str()),
        "8 5 12 12 15 23 15 18 12 4"
    );
    assert_eq!(digital_root(16), 7);
    assert_eq!(count_duplicates("abcde"), 0);
    assert_eq!(count_chars("aba"), map_key_val!('a': 2, 'b': 1));
    assert_eq!(
        unique_in_order("AAAABBBCCDAABBB".chars()),
        vec!['A', 'B', 'C', 'D', 'A', 'B']
    );
}
