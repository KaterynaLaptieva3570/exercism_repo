pub fn abbreviate(phrase: &str) -> String {
    let mut result = String::new();

    // Replace hyphens with spaces
    let cleaned = phrase.replace('-', " ");

    for word in cleaned.split_whitespace() {
        let letters: Vec<char> = word
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect();

        if letters.is_empty() {
            continue;
        }

        // Always take the first letter
        result.push(letters[0].to_ascii_uppercase());

        // CamelCase
        let all_upper = word.chars().all(|c| !c.is_lowercase());
        if !all_upper {
            for &c in &letters[1..] {
                if c.is_uppercase() {
                    result.push(c);
                }
            }
        }
    }

    result
}
