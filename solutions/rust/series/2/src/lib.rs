pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec!["".to_string(); digits.len() + 1];
    }

    let chars: Vec<char> = digits.chars().collect();
    if len > chars.len() {
        return Vec::new();
    }

    let mut result = Vec::new();
    for i in 0..=chars.len() - len {
        let s: String = chars[i..i + len].iter().collect();
        result.push(s);
    }

    result
}
