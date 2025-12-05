fn talk(msg: &str) -> bool {
    let mut has_letters = false;
    for c in msg.chars() {
        if c.is_alphabetic() {
            has_letters = true;
            break;
        }
    }
    msg.to_uppercase() == msg && has_letters
}

pub fn reply(msg: &str) -> &str {
    let m = msg.trim();

    if m.len() == 0 {
        return "Fine. Be that way!";
    } else if m.ends_with("?") && talk(m) {
        return "Calm down, I know what I'm doing!";
    } else if m.ends_with("?") {
        return "Sure.";
    } else if talk(m) {
        return "Whoa, chill out!";
    } else {
        return "Whatever.";
    }
}
