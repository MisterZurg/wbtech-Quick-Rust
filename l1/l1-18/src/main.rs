use l1_18::reverse_line;

fn scan_input() -> String{
    let mut line = String::new();
    println!("Input line to reverse >");

    std::io::stdin()
        .read_line(&mut line)
        .unwrap();

    line
}

fn main() {
    let line = scan_input();

    println!("{}", reverse_line(line));
}
