#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    // check bases 
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    // empty input 
    if number.is_empty() {
        return Ok(vec![0]);
    }

    // input to decimal
    let mut value: u128 = 0;
    for &digit in number {
        if digit >= from_base {
            return Err(Error::InvalidDigit(digit));
        }
        value = value * (from_base as u128) + digit as u128;
    }

    // from decimal to target base
    if value == 0 {
        return Ok(vec![0]);
    }

    let mut result = Vec::new();
    let mut n = value;
    while n > 0 {
        result.push((n % to_base as u128) as u32);
        n /= to_base as u128;
    }

    result.reverse();
    Ok(result)
}