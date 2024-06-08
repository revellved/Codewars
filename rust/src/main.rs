use alphabet_position::alphabet_position;
use codewars::map_key_val;
use count_chars::count_chars;
use count_duplicates::count_duplicates;
use digital_root::digital_root;
use hello::hello;
use phone_number::create_phone_number;
use tralling_zeros::zeros;
use unique_in_order::unique_in_order;
use valid_isbn::valid_isbn10;
use your_order_please::order;

mod alphabet_position;
mod count_chars;
mod count_duplicates;
mod digital_root;
mod hello;
mod phone_number;
mod tralling_zeros;
mod unique_in_order;
mod valid_isbn;
mod your_order_please;

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
    assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
    assert_eq!(
        create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
        "(123) 456-7890"
    );
    assert_eq!(zeros(6), 1);
    assert_eq!(zeros(14), 2);
    assert_eq!(valid_isbn10("1112223339"), true);
}
