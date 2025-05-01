pub fn reverse_words(line: String) -> String {
    line.split_whitespace() // Step 1: Split by whitespace
        .rev() // Step 2: Reverse the order
        .collect::<Vec<&str>>() // Collect into a vector
        .join(" ") // Step 3: Join with spaces
}
