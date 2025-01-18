#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }

    if let Some(v) = string_digits.chars().find(|c| !c.is_ascii_digit()) {
        return Err(Error::InvalidDigit(v));
    }

    Ok(string_digits
        .chars()
        .collect::<Vec<char>>()
        .windows(span)
        .fold(0, |acc, chars| {
            acc.max(
                chars
                    .to_vec()
                    .iter()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
                    .iter()
                    .fold(1, |acc, curr| acc * (*curr as u64)),
            )
        }))
}
