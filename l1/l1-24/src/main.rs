use std::collections::HashSet;

fn is_unique(line: String) -> bool {
    let mut set = HashSet::new();

    for c in line.chars() {
        if set.get(&c).is_some() {
            return false
        }
        set.insert(c);
    }

    true
}

fn main() {
    println!("{}", is_unique("Hello, world!".to_string()));
    println!("{}", is_unique("abcd".to_string()));
    println!("{}", is_unique("abCdefAaf".to_string()));
    println!("{}", is_unique("aabcd".to_string()));
    println!("{}", is_unique("Hotel".to_string()));
    println!("{}", is_unique("Trivago".to_string()));
}
