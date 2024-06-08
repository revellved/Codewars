use alphabet_position::alphabet_position;
use hello::hello;

mod alphabet_position;
mod hello;

fn main() {
    let text = hello().clone();
    println!("{text}");

    assert_eq!(
        alphabet_position(text.as_str()),
        "8 5 12 12 15 23 15 18 12 4"
    );
}
