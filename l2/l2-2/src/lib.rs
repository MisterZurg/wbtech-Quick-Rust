use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct UnpackError;

impl fmt::Display for UnpackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(некорректная строка)")
    }
}

/// unpack_string — осуществляющую примитивную распаковку строки,
/// содержащую повторяющиеся символы.
pub fn unpack_string(packed: &String) -> Result<String, UnpackError> {
    if packed == "" {
        return Ok(packed.to_string())
    }

    if !packed.chars().nth(0).unwrap().is_alphabetic() {
        return Err(UnpackError);
    }

    let mut decum: Vec<String> = Vec::new();

    let mut i:usize = 0;



    while i < packed.len() {
        let mut j = i + 1;

        let mut count_repeats: Vec<String> = Vec::new();
        while j < packed.len() && packed.chars().nth(j).unwrap().is_digit(10) {
            count_repeats.push(String::from(packed.chars().nth(j).unwrap()));
            j += 1;
        }

        let mut repeat: i32 = 1;
        if count_repeats.len() > 0 {
            repeat = count_repeats.join("").parse().unwrap();
        }

        for _ in 0..repeat {
            decum.push(packed.chars().nth(i).unwrap().to_string());
        }

        i = j;
    }

    Ok(decum.join(""))
}