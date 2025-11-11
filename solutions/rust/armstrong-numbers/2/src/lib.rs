pub fn is_armstrong_number(num: u32) -> bool {
    // change the number to a string so we can look at each digit
    let num_string = num.to_string();
    let mut digits: Vec<u32> = Vec::new();

    // go through every character in the string
    for ch in num_string.chars() {
        let digit = ch.to_digit(10).unwrap();
        digits.push(digit);
    }

    // find how many digits the number has
    let power = digits.len() as u32;

    // now add up each digit raised to the power
    let mut sum = 0;
    for d in &digits {
        sum += d.pow(power);
    }

    // check if the sum is same as the number
    if sum == num {
        return true;
    } else {
        return false;
    }
}
