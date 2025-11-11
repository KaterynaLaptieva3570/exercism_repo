use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut seen = HashSet::new();

    for ch in candidate.chars().filter(|c| c.is_alphabetic()) {
        let lower = ch.to_ascii_lowercase();
        if !seen.insert(lower) {
    
            return false;
        }
    }

    true
}
