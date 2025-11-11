use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();

    // lowercase + sorted target word
    let lower_target = word.to_lowercase();
    let mut sorted_target: Vec<char> = lower_target.chars().collect();
    sorted_target.sort_unstable();

    for &candidate in possible_anagrams {
        let lower_candidate = candidate.to_lowercase();

        // skip same word
        if lower_candidate == lower_target {
            continue;
        }

        // sort candidate letters
        let mut sorted_candidate: Vec<char> = lower_candidate.chars().collect();
        sorted_candidate.sort_unstable();

        // if same sorted letters → anagram
        if sorted_candidate == sorted_target {
            result.insert(candidate);
        }
    }

    result
}

