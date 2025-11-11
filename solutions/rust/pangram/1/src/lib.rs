use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    let letters: HashSet<char> = sentence
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect();

    letters.len() == 26
}
