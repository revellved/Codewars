pub fn hello() -> String {
    "Hello World!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_hello() {
        assert_eq!(hello(), "Hello World!");
    }
}
