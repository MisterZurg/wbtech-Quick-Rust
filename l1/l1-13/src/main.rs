fn print_with_no_duplicates(txt: Vec<&str>) -> Vec<&str> {
    let mut unique_txt = Vec::new();

    for line in txt {
        if !unique_txt.contains(&line) {
            unique_txt.push(line);
        }
    }

    unique_txt
}


fn main() {
    let txt = vec![
        "Ona chce być jak Mona Lisa",
        "Mam cash i kartę VISA",
        "Moje życie loca la vida",
        "(Oo, oo, oohh)",
        "Ona chce być jak Mona Lisa",
        "Mam cash i kartę VISA",
        "Moje życie loca la vida",
        "Hej suczki, rararara",
        "Znamy wasze sztuczki rarararara",
        "Hej suczki, rararara",
        "Znamy wasze sztuczki rarararara",
    ];

    for line in print_with_no_duplicates(txt) {
        println!("{line}");
    }
}
