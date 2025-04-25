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

fn add_before<'a>(before: &usize, curr_idx: usize, lines: &Vec<&'a str>, results: &mut Vec<&'a str>) {
    if *before == 0 {
        return;
    }

    let left_guard = if curr_idx-before > 0 {curr_idx-before} else { 0 };
    for j in left_guard..curr_idx {
        results.push(lines[j]);
    }
}

fn add_after<'a>(after: &usize, curr_idx: usize, lines: &Vec<&'a str>, results: &mut Vec<&'a str>) {
    if *after == 0 {
        return;
    }

    let right_guard = if curr_idx-after > 0 { curr_idx-after } else { 0 };
    for j in curr_idx..right_guard {
        results.push(lines[j]);
    }
}

fn search<'a>(cfg: &Config, lines: Vec<&'a str>) -> Vec<&'a str> {
    let mut results = Vec::new();

    let left_boarder =  if cfg.context > 0 { cfg.context } else { cfg.before };
    let right_boarder =  if  cfg.context > 0 { cfg.context } else { cfg.after };

    for i in 0..lines.len() {
        if lines[i].contains(&cfg.query) {
            // do something with line
            add_before(&left_boarder, i, &lines, &mut results);

            results.push(lines[i]);

            add_after(&right_boarder, i, &lines, &mut results)
        }
    }

    results
}

/// cargo run -- -q 'Забрал десять по десять'
fn search_exact<'a>(cfg: &Config, lines: Vec<&'a str>) -> Vec<&'a str> {
    let mut results = Vec::new();

    for i in 0..lines.len() {
        if lines[i].eq(&cfg.query) {
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
    // let found = search_exact(&cfg, lines);
    if cfg.count_lines {
        println!("{:?}", found.len());
    }
    println!("{:?}", found);
}
