use alphabet_position::alphabet_position;
use digital_root::digital_root;
use hello::hello;

mod alphabet_position;
mod digital_root;
mod hello;

fn main() {
    let text = hello().clone();
    println!("{text}");

    assert_eq!(
        alphabet_position(text.as_str()),
        "8 5 12 12 15 23 15 18 12 4"
    );
    assert_eq!(digital_root(16), 7);
}
