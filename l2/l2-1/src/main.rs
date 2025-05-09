use std::env::{args};
use std::error::Error;
use std::fs;
use std::fs::File;
use log::{error, warn};

struct Config {
    flag: String,
    file_path: String
}

impl Config{
    fn new(args: &[String]) -> Self {
        if args.len() == 0 {
            error!("ЧЗХ мало аргументов");
            return Config { flag : "".to_string(), file_path: "".to_string()}
        }

        let flag: String = args[1].clone();
        let file_path: String = args[2].clone();

        Config{ flag, file_path }
    }
    // fn of(args: &[String]) -> Result<Config, Err> {
    //     if args.len() == 0 {
    //         return Err("ЧЗХ мало аргументов")
    //     }
    //
    //     let flag: String = args[1].clone();
    //     let file_path: String = args[2].clone();
    //
    //     Ok(Config{ flag, file_path })
    // }
}

fn run(cfg: Config) -> Result<(), Box<dyn Error>>{
    // Get file content "./l2/l2-1/test.txt"
    println!("{}", &cfg.file_path);

    let contents = fs::read_to_string(&cfg.file_path.as_str())?;
    let result: usize = match cfg.flag.as_str() {
        "-c" => { count_symbols(contents) },
        "-l" => { count_lines(contents) },
        // "-b" => { contents.bytes().count() },
        "-w" => { count_words(contents) },
        _ => { panic!("ЧЗХ")},
    };

    println!("{} {}", result, cfg.file_path);

    Ok(())
}

/// count_symbols — показать количество символов в файле при использовании -с
fn count_symbols(cnt: String) -> usize {
    cnt.chars().count()
}

/// count_lines — вывести количество строк в файле при использовании -l
fn count_lines(cnt: String) -> usize {
    cnt.lines().count()
}

/// count_words — отобразить количество слов в файле при использовании -w
fn count_words(cnt: String) -> usize {
    cnt.split_ascii_whitespace().count()
}

fn main() {
    // args() — Returns the arguments that this program was started with (normally passed via the command line).
    let cli_args: Vec<String> = args().collect();

    let cfg = Config::new(&cli_args);
    
    run(cfg).expect("TODO: panic message");
}
