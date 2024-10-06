use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;

use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
struct Config {
    in_file: String,
    #[clap(default_value = "out.txt")]
    out_file: String,

    /// -k — указание колонки для сортировки
    #[clap(short = 'k', long, default_value = "1")]
    column : usize,
    /// -n — сортировать по числовому значению
    #[clap(short = 'n', long)]
    numeric : bool,
    /// -r — сортировать в обратном порядке
    #[clap(short = 'r', long)]
    reverse: bool,
    /// -u — не выводить повторяющиеся строки
    #[clap(short = 'u', long)]
    unique: bool,
    /// -M — сортировать по названию месяца
    #[clap(short = 'M', long)]
    sort_by_month: bool,
    /// -b — игнорировать хвостовые пробелы
    #[clap(short = 'b', long)]
    ignore_trailing_spaces: bool,
    /// -c — проверять отсортированы ли данные
    #[clap(short = 'c', long)]
    is_sorted: bool,
    /// s (-h) — сортировать по числовому значению с учетом суффиксов
    #[clap(short = 's', long)]
    sort_by_suffix: bool,
}

// cargo run -- -- l2/l2-3/sort-me.txt l2/l2-3/out.txt
fn main() {
    let cfg = Config::parse();

    // Чтение input файла
    let content = fs::read_to_string(&cfg.in_file)
        .expect("Cannot read in_file");

    // Сплитим на строчки
    let mut lines: Vec<&str> = content.lines().collect();

    // remove_duplicates(&mut lines);
    // reverse_lines(&mut lines);

    lines.sort_by(|a, b| {
        let a_key = get_column(a, cfg.column, cfg.ignore_trailing_spaces);
        let b_key = get_column(b, cfg.column, cfg.ignore_trailing_spaces);

        let cmp_result = if cfg.sort_by_month {
            compare_as_month(&a_key, &b_key)
        } else if cfg.sort_by_suffix {
            compare_as_human_numbers(&a_key, &b_key)
        } else if cfg.numeric {
            compare_as_numbers(&a_key, &b_key)
        } else {
            a_key.cmp(&b_key)
        };

        // Сортировка в обратном порядке (если опция -r включена)
        if cfg.reverse {
            reverse_lines(cmp_result)
        }

        cmp_result
    });

    // Запись результата в output файл
    fs::write(&cfg.out_file, lines.join("\n")).expect("Не удалось записать в output файл");
}


/// remove_duplicates() helper for flag
/// https://stackoverflow.com/questions/47636618/vecdedup-does-not-work-how-do-i-deduplicate-a-vector-of-strings
fn remove_duplicates(lines: &mut Vec<&str>) {
    // let mut unique: HashSet<&str> = HashSet::new();
    let mut uniques: HashSet<&str> = HashSet::new();
    lines.retain(|e| uniques.insert(*e));
}

fn reverse_lines(lines: Ordering) {
    lines.reverse();
}

/// Проверка, отсортированы ли строки
fn is_sorted(lines: &[&str], cfg: &Config) -> bool {
    for i in 1..lines.len() {
        let a_key = get_column(lines[i - 1], cfg.column, cfg.ignore_trailing_spaces);
        let b_key = get_column(lines[i], cfg.column, cfg.ignore_trailing_spaces);

        let cmp_result = if cfg.sort_by_month {
            compare_as_month(&a_key, &b_key)
        } else if cfg.sort_by_suffix {
            compare_as_human_numbers(&a_key, &b_key)
        } else if cfg.numeric {
            compare_as_numbers(&a_key, &b_key)
        } else {
            a_key.cmp(&b_key)
        };

        if cmp_result != Ordering::Less && !cfg.reverse
            || cmp_result != Ordering::Greater && cfg.reverse {
            return false;
        }
    }
    true
}

/// Извлечение колонки для сортировки
fn get_column(line: &str, column: usize, ignore_trailing_spaces: bool) -> String {
    let line = if ignore_trailing_spaces {
        line.trim_end()
    } else {
        line
    };

    line.split_whitespace()
        .nth(column - 1)
        .unwrap_or("")
        .to_string()
}

/// Сравнение строк как чисел
fn compare_as_numbers(a: &str, b: &str) -> Ordering {
    let a_num = a.parse::<i32>();
    let b_num = b.parse::<i32>();

    match (a_num, b_num) {
        (Ok(a), Ok(b)) => a.partial_cmp(&b).unwrap_or(Ordering::Equal),
        (Ok(_), Err(_)) => Ordering::Less,
        (Err(_), Ok(_)) => Ordering::Greater,
        (Err(_), Err(_)) => a.cmp(b),
    }
}

/// Сравнение строк как месяцев
fn compare_as_month(a: &str, b: &str) -> Ordering {
    let months = [
        "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
    ];
    let a_lower = a.to_lowercase();
    let b_lower = b.to_lowercase();
    let a_idx = months.iter().position(|&m| m == a_lower);
    let b_idx = months.iter().position(|&m| m == b_lower);

    match (a_idx, b_idx) {
        (Some(a), Some(b)) => a.cmp(&b),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => a.cmp(b),
    }
}

/// Сравнение строк как "человеческие" числа (например "10K", "5M")
fn compare_as_human_numbers(a: &str, b: &str) -> Ordering {
    let re = Regex::new(r"^(\d+)([KMGTP]?)$").unwrap();

    let parse_human_number = |s: &str| -> Option<i32> {
        if let Some(captures) = re.captures(s) {
            let num = captures[1].parse::<i32>().ok()?;
            let suffix = &captures[2];
            let multiplier = match suffix {
                "K" => 1_000,
                "M" => 1_000_000,
                "G" => 1_000_000_000,
                _ => 1,
            };
            return Some(num * multiplier);
        }
        None
    };

    let a_num = parse_human_number(a).unwrap_or(0);
    let b_num = parse_human_number(b).unwrap_or(0);

    a_num.partial_cmp(&b_num).unwrap_or(Ordering::Equal)
}