use std::fs;
use clap::Parser;

#[derive(Parser, Debug)]
struct Config {
    #[clap(default_value = "l2/l2-5/poem.txt")]
    in_file: String,
    #[clap(short = 'q')]
    query: String,
    /// -A — "after" печатать +N строк после совпадения
    #[clap(short = 'A', long, default_value = "0")]
    after: usize,
    /// -B — "before" печатать +N строк до совпадения
    #[clap(short = 'B', long, default_value = "0")]
    before: usize,
    /// -C — "context" (A+B) печатать ±N строк вокруг совпадения
    #[clap(short = 'C', long, default_value = "0")]
    context: usize,
    /// -c — "count" (количество строк)
    #[clap(short = 'c', long, default_value = "false")]
    count_lines: bool,
    /// -i — "ignore-case" (игнорировать регистр)
    #[clap(short = 'i', long, default_value = "false")]
    ignore_case: bool,
    /// -v — "invert" (вместо совпадения, исключать)
    #[clap(short = 'v', long, default_value = "false")]
    invert: bool,
    /// -F — "fixed", точное совпадение со строкой, не паттерн
    #[clap(short = 'F', long, default_value = "false")]
    fixed: bool,
    /// -n — "line num", напечатать номер строки
    #[clap(short = 'n', long, default_value = "false")]
    print_line_num: bool,
}

impl Config {
    fn new() -> Config {
        Config::parse()
    }
}

fn search<'a>(cfg: &Config, lines: Vec<&'a str>) -> Vec<&'a str> {
    let mut results = Vec::new();

    let mut before = cfg.before;
    let mut after= cfg.after;

    for i in 0..lines.len() {
        if lines[i].contains(&cfg.query) {
            // do something with line
            results.push(lines[i]);
        }
    }

    results
}

fn main() {
    let cfg = Config::new();

    // Чтение input файла
    let content = fs::read_to_string(&cfg.in_file)
        .expect("Cannot read in_file");

    // Сплитим на строчки
    let lines: Vec<&str> = content
        .lines()
        .collect();

    let found = search(&cfg, lines);

    println!("{:?}", found);
}
