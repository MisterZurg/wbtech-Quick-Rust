use std::fs;
use reqwest;

const URL: &str = "https://tech.wildberries.ru/courses/rust";

fn main() {
    // download the target HTML document
    let response = reqwest::blocking::get(URL);
    // get the HTML content from the request response
    // and print it
    let html_content = response
        .unwrap()
        .text()
        .unwrap()
        .split("><")
        .collect::<Vec<_>>()
        .join(">\n<");

    // Запись результата в output файл
    fs::write("./l2/l2-9/downloaded.html", html_content)
        .expect("Не удалось записать в output файл");
}