pub fn annotate(garden: &[&str]) -> Vec<String> {
    let rows = garden.len();
    if rows == 0 {
        return vec![];
    }

    let cols = garden[0].len();
    let mut result = Vec::new();

    for r in 0..rows {
        let mut row_string = String::new();
        for c in 0..cols {
            let cell = garden[r].as_bytes()[c];
            if cell == b'*' {
                row_string.push('*');
            } else {
                let mut count = 0;
                // check all 8 directions around the cell
                for dr in [-1, 0, 1] {
                    for dc in [-1, 0, 1] {
                        if dr == 0 && dc == 0 {
                            continue;
                        }
                        let nr = r as isize + dr;
                        let nc = c as isize + dc;
                        if nr >= 0 && nc >= 0 && (nr as usize) < rows && (nc as usize) < cols {
                            if garden[nr as usize].as_bytes()[nc as usize] == b'*' {
                                count += 1;
                            }
                        }
                    }
                }
                if count == 0 {
                    row_string.push(' ');
                } else {
                    row_string.push(char::from_digit(count, 10).unwrap());
                }
            }
        }
        result.push(row_string);
    }

    result
}
