pub fn reply(message: &str) -> &str {
    let msg = message.trim();

    if msg.is_empty() {
        return "Fine. Be that way!";
    }

    let is_question = msg.ends_with('?');
    let has_letters = msg.chars().any(|c| c.is_ascii_alphabetic());
    let is_yelling = has_letters && msg.chars().filter(|c| c.is_ascii_alphabetic()).all(|c| c.is_uppercase());

    match (is_question, is_yelling) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Sure.",
        (false, true) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
