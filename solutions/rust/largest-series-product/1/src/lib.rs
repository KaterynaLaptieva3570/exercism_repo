#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }
    if span == 0 {
        return Ok(1);
    }

    let digits: Vec<u64> = string_digits
        .chars()
        .map(|c| {
            c.to_digit(10)
                .map(|d| d as u64)
                .ok_or(Error::InvalidDigit(c))
        })
        .collect::<Result<_, _>>()?;

    let mut best = 0;
    for w in digits.windows(span) {
        let p = w.iter().product();
        if p > best {
            best = p;
        }
    }
    Ok(best)
}
