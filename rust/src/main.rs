fn hello() -> String {
    "HELL WORLD!!!".to_string()
}

fn main() {
    println!("{}", hello());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_hello() {
        assert_eq!(hello(), "Hello World!");
    }
}
