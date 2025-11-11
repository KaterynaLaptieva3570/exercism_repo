pub fn answer(command: &str) -> Option<i32> {
    // Must start and end properly
    if !command.starts_with("What is ") || !command.ends_with('?') {
        return None;
    }

    // Clean question text
    let mut words = command
        .trim_end_matches('?')
        .trim_start_matches("What is ")
        .replace(" by", ""); // normalize “divided by”, “multiplied by”

    if words.is_empty() {
        return None;
    }

    let mut tokens = words.split_whitespace();
    let mut result = tokens.next()?.parse::<i32>().ok()?; // first number

    while let Some(op) = tokens.next() {
        let next_token = tokens.next()?;
        let num = next_token.parse::<i32>().ok()?;

        result = match op {
            "plus" => result + num,
            "minus" => result - num,
            "multiplied" => result * num,
            "divided" => result / num,
            _ => return None, // unsupported operation
        };
    }

    Some(result)
}
