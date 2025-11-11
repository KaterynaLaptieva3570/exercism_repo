/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // Remove spaces
    let code: String = code.chars().filter(|c| !c.is_whitespace()).collect();

    // Must be at least 2 digits
    if code.len() <= 1 {
        return false;
    }

    // Check all characters are digits
    if !code.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    // Go from right to left, doubling every second digit
    let mut sum = 0;
    let mut double = false;

    for ch in code.chars().rev() {
        let mut num = ch.to_digit(10).unwrap();

        if double {
            num *= 2;
            if num > 9 {
                num -= 9;
            }
        }

        sum += num;
        double = !double;
    }

    // Valid if sum divisible by 10
    sum % 10 == 0
}
