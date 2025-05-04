use l1_18::reverse_line;

#[test]
fn test_empty() {
    assert_eq!(reverse_line("".to_string()), "")
}

#[test]
fn test_example() {
    assert_eq!(reverse_line("главрыба".to_string()), "абырвалг")
}

#[test]
fn test_space() {
    assert_eq!(reverse_line("реклама бобс".to_string()), "сбоб амалкер")
}
