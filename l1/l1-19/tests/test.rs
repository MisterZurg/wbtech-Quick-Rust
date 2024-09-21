use l1_19::reverse_words;
#[test]
fn test_example() {
    assert_eq!(reverse_words("snow dog sun".to_string()), "sun dog snow");
}

#[test]
fn test_empty() {
    assert_eq!(reverse_words("".to_string()), "")
}

#[test]
fn test_one_line() {
    assert_eq!(reverse_words("zeliboba".to_string()), "zeliboba")
}
