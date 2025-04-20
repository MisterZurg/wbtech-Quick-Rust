use l2_2::unpack_string;

fn main() {
    /*
        "a4bc2d5e" => "aaaabccddddde"
        "abcd" => "abcd"
        "45" => "" (некорректная строка)
        "" => ""
     */
    match unpack_string(&"45".to_string()) {
        Ok(dc) => println!("{dc}"),
        Err(e) =>  println!("{e}")
    }
}
