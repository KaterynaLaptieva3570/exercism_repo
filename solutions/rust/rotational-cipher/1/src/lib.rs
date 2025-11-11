pub fn rotate(input: &str, key: u8) -> String {
    let k = key % 26;

    input
        .chars()
        .map(|c| {
            match c {
                'a'..='z' => {
                    let b = (c as u8 - b'a' + k) % 26 + b'a';
                    b as char
                }
                'A'..='Z' => {
                    let b = (c as u8 - b'A' + k) % 26 + b'A';
                    b as char
                }
                _ => c,
            }
        })
        .collect()
}
