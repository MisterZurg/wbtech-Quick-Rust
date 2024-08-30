fn forty_two() -> i32 {
    return 42
}

fn main() {
    forty_two();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forty_two() {
        assert_eq!(forty_two(), 42);
    }

    #[test]
    #[should_panic(expected = "42")]
    fn test_should_fail() {
        assert_eq!(forty_two(), 228);
    }
}