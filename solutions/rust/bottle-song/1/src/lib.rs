pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut current_bottles = start_bottles;
    let mut verses = Vec::new();

    for _ in 0..take_down {
        let current = capitalize(number_str_mapper(current_bottles));
        let next = number_str_mapper(current_bottles - 1);
        let verse = format!(
            "{current} hanging on the wall,\n\
             {current} hanging on the wall,\n\
             And if one green bottle should accidentally fall,\n\
             There'll be {next} hanging on the wall.\n"
        );
        verses.push(verse);
        current_bottles -= 1;
    }

    verses.join("\n")
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

const MAPPINGS: [&str; 11] = [
    "no green bottles",
    "one green bottle",
    "two green bottles",
    "three green bottles",
    "four green bottles",
    "five green bottles",
    "six green bottles",
    "seven green bottles",
    "eight green bottles",
    "nine green bottles",
    "ten green bottles",
];

fn number_str_mapper(bottles: u32) -> &'static str {
    MAPPINGS[bottles as usize]
}
