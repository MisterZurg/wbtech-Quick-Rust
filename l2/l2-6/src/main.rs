use std::io;
use std::io::BufRead;
use clap::Parser;

#[derive(Parser, Debug)]
struct Config {
    /// -f — "fields" - выбрать поля (колонки)
    #[clap(short = 'f', long)]
    fields : String,
    /// -d — "delimiter" - использовать другой разделитель
    #[clap(short = 'd', long)]
    delimiter : String,
    /// -s — "separated" - только строки с разделителем
    #[clap(short = 's', long)]
    separated: bool,
}

impl Config {
    fn new() -> Config {
        Config::parse()
    }
}

fn main() {
    let cfg = Config::new();

    // Парсинг поля
    let fields: Vec<usize> = cfg
        .fields
        .split(',')
        .filter_map(|f| f.parse::<usize>().ok())
        .collect();

    // Read lines from stdin
    let stdin = io::stdin();

    for line in stdin.lock().lines().map(|line| line.unwrap()) {
        // Fuck up check, if line contains separator
        if cfg.separated && !line.contains(&cfg.delimiter) {
            continue;
        }

        // Split by separator
        let columns: Vec<&str> = line.split(&cfg.delimiter).collect();

        // Select queried cols
        let selected_fields: Vec<&str> = fields
            .iter()
            .filter_map(|field| columns.get(field - 1))
            .cloned()
            .collect();

        // Result
        if !selected_fields.is_empty() {
            println!("{}", selected_fields.join(&cfg.delimiter));
        }
    }
}
