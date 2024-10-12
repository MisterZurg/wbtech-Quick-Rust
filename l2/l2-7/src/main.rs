use std::fs;
use std::collections::HashMap;
use std::ops::AddAssign;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use clap::Parser;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

#[derive(Parser, Debug)]
struct Config {
    #[clap(default_value = "l2/l2-7/copypasta.txt")]
    in_file: String,

    /// -t — количество потоков, для подсчета частоты встречаемости символов
    #[clap(short = 't', default_value = "1")]
    n_threads: usize,
}

impl Config {
    fn new() -> Config {
        Config::parse()
    }
}

#[derive(Debug)]
struct Frequency {
    elapsed: u64,
    lf: HashMap<char, usize>,
}

/// Parallel Letter Frequency https://exercism.org/tracks/rust/exercises/parallel-letter-frequency
fn letter_frequency(lines: &str) -> Frequency {
    let time_start = Instant::now();

    let freq_map = Arc::new(Mutex::new(HashMap::new()));

    lines.par_chars()
        // фильтруем только латинские символы
        .filter(|c| c.is_ascii_alphabetic())
        // приводим к нижнему регистру
        .map(|c| c.to_ascii_lowercase())
        .for_each_with(freq_map.clone(), |map, c| {
            // подсчёт символов
            map
                .lock()
                .unwrap()
                .entry(c)
                .or_insert(0)
                .add_assign(1)
        });

    let time_elapsed =  time_start.elapsed().as_secs();
    let map = Arc::try_unwrap(freq_map)
        .unwrap().
        into_inner()
        .unwrap();

    Frequency { elapsed: time_elapsed, lf: map }
}

fn main() {
    let cfg = Config::new();

    // Чтение input файла
    let content = fs::read_to_string(&cfg.in_file)
        .expect("Cannot read in_file");

    // Used to create a new ThreadPool or to configure the global rayon thread pool.
    let _ = ThreadPoolBuilder::new()
        .num_threads(cfg.n_threads)
        .build_global()
        .unwrap();

    let lf = letter_frequency(&content);

    println!("{:+?}", lf);
}
