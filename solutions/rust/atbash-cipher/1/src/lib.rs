/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut result = String::new();
    let mut count = 0;

    for c in plain.chars() {
        if c.is_ascii_alphanumeric() {
            let mapped = match c.to_ascii_lowercase() {
                'a'..='z' => (b'z' - (c.to_ascii_lowercase() as u8 - b'a')) as char,
                d if d.is_ascii_digit() => d,
                _ => continue,
            };

            result.push(mapped);
            count += 1;

            if count % 5 == 0 {
                result.push(' ');
            }
        }
    }

    result.trim_end().to_string()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter_map(|c| {
            if c.is_ascii_alphanumeric() {
                Some(match c {
                    'a'..='z' => (b'z' - (c as u8 - b'a')) as char,
                    d if d.is_ascii_digit() => d,
                    _ => return None,
                })
            } else {
                None
            }
        })
        .collect()
}
