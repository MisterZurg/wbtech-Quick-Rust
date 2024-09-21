fn remove_ith<T: Sized + std::fmt::Debug>(sl: &mut Vec<T>, idx: usize) {
    if sl.get(idx).is_some() {
        sl.remove(idx);
    }
}

fn main() {
    let mut str_slice = vec!["W", "B", "T", "E", "C", "H"];
    let mut num_slice = vec![1, 2, 3, 4, 5];

    remove_ith(&mut str_slice, 2);
    remove_ith(&mut num_slice, 2);

    println!("{:?}", str_slice);
    println!("{:?}", num_slice);

    let mut fl_slice = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    remove_ith(&mut fl_slice, 99);
    println!("{:?}", fl_slice);
}
