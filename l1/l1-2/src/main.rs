use std::{io, thread};

/// scan_vector_size helper for init_vec()
fn scan_vector_size() -> usize {
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

/// init_vec create initialized vector with nums in range 1..size
fn init_vec(size: usize) -> Vec<u128> {
    let mut arr: Vec<u128> = vec![0; size];
    for i in 0..arr.len() {
        arr[i] = (i + 1) as u128
    }

    arr
}

fn main() {
    let n = scan_vector_size();
    let arr = init_vec(n);
    // println!("{:?}", arr);

    // Create a thread
    let square_handle = thread::spawn(move || {
        // Everything in here runs in a separate thread
        for i in 0..arr.len() {
            print!("{} ", arr[i] * arr[i]);
        }
    });

    // Wait until other thread has finished
    square_handle.join().expect("Thread did not panic");
}
