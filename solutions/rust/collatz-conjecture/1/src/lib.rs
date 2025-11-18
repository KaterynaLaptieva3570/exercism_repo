pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut steps = 0;
    let mut value = n;

    while value != 1 {
        if value % 2 == 0 {
            value /= 2;
        } else {
            value = value.checked_mul(3)?.checked_add(1)?;
        }
        steps += 1;
    }

    Some(steps)
}
