use std::io;

/// scan_vector_size helper for init_vec()
fn scan_bit_to_set() -> usize {
    let mut input_line = String::new();

    io::stdin() // the rough equivalent of `std::cin`
        .read_line(&mut input_line) // actually read the line
        .expect("Failed to read line"); // which can fail, however

    let x: usize = input_line
        .trim() // ignore whitespace around input
        .parse() // convert to integers
        .expect("Input not an integer"); // which, again, can fail

    x
}

/// set_bit helper for setting certain bit
fn set_bit(mut number: i64, index: usize, value: bool) -> i64 {
    if value {
        // Set the i-th bit to 1
        number |= 1 << index;
    } else {
        // Set the i-th bit to 0
        number &= !(1 << index);
    }

    number
}

fn main() {
    let set_i_bit: i64 = 10; // Example number
    let index: usize = scan_bit_to_set(); // Bit position to modify
    let set_to_one: bool = true; // Change this to false to set to 0

    let result = set_bit(set_i_bit, index, set_to_one);

    println!("Result after setting bit: {}", result); // Output the result
}
