pub fn is_valid_isbn(isbn: &str) -> bool {
    let clean: String = isbn.chars().filter(|c| *c != '-').collect();

    if clean.len() != 10 {
        return false;
    }

    let mut sum = 0;
    for (i, ch) in clean.chars().enumerate() {
        let position = (10 - i) as u32;
        let value = match ch {
            '0'..='9' => ch.to_digit(10).unwrap(),
            'X' if i == 9 => 10,
            _ => return false,
        };
        sum += value * position;
    }

    sum % 11 == 0
}
