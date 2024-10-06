use std::collections::HashMap;
use itertools::Itertools;

/// Anagrams — helper structure to store anagrams
/// sorted_by_chars_anagram -> words
pub struct Anagrams(HashMap<String, Vec<String>>);

impl Anagrams{
    fn new() -> Anagrams {
        Anagrams(HashMap::new())
    }

    fn insert(&mut self, word: String) {
        let anagram_key = Self::get_anagram(word.clone());

        self.0
            .entry(anagram_key)
            .or_insert_with(Vec::new)
            .push(word);
    }

    fn get(&self, word: String) -> Vec<String> {
        let anagram_key = Self::get_anagram(word.clone());

        match self.0.get(&anagram_key) {
            Some( anagrams) => anagrams.clone(),
            None => Vec::new(),
        }
    }

    fn get_anagram(word : String) -> String {
        word.chars().sorted().collect::<String>()
    }
}

fn main() {
    let mut anagrams = Anagrams::new();
    // "пятак", "пятка", "тяпка"
    anagrams.insert("пятак".to_string());
    anagrams.insert("пятка".to_string());
    anagrams.insert("тяпка".to_string());

    println!(
        "{:?}",
        anagrams.get("тяпка".to_string())
    );

    //  "листок", 'слиток' и 'столик'
    anagrams.insert("листок".to_string());
    anagrams.insert("слиток".to_string());
    anagrams.insert("столик".to_string());

    println!(
        "{:?}",
        anagrams.get("слиток".to_string())
    );
}