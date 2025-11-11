pub fn number(user_number: &str) -> Option<String> {
    // Keep only digits
    let mut digits: String = user_number.chars().filter(|c| c.is_ascii_digit()).collect();

    // Remove country code prefix "1" if present (valid only if total length is 11)
    if digits.len() == 11 && digits.starts_with('1') {
        digits.remove(0);
    }

    // Must now be exactly 10 digits
    if digits.len() != 10 {
        return None;
    }

    // Convert to bytes for digit checks
    let bytes = digits.as_bytes();

    // Area code (first digit) cannot be 0 or 1
    // Exchange code (fourth digit) cannot be 0 or 1
    if bytes[0] < b'2' || bytes[3] < b'2' {
        return None;
    }

    Some(digits)
}
