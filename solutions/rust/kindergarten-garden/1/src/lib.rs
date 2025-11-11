pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred",
        "Ginny", "Harriet", "Ileana", "Joseph", "Kincaid", "Larry",
    ];

    let index = students.iter().position(|&s| s == student).unwrap();

    let rows: Vec<&str> = diagram.lines().collect();
    let mut result = Vec::new();

    for row in &rows {
        let chars: Vec<char> = row.chars().collect();
        result.push(plant_name(chars[2 * index]));
        result.push(plant_name(chars[2 * index + 1]));
    }

    result
}

fn plant_name(c: char) -> &'static str {
    match c {
        'G' => "grass",
        'C' => "clover",
        'R' => "radishes",
        'V' => "violets",
        _ => "unknown",
    }
}
