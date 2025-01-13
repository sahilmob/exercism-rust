/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn = isbn.chars().filter(|c| c != &'-');
    let count = isbn.clone().count();

    if count != 10 {
        return false;
    }

    match isbn.enumerate().try_fold(0, |acc, (i, v)| match v {
        v if i == count - 1 && v == 'X' => Ok(acc + 10),
        v if v.is_alphabetic() => Err(()),
        v => Ok(v
            .to_digit(10)
            .map(|digit| acc + (digit * (10 - i) as u32))
            .unwrap()),
    }) {
        Ok(v) => v % 11 == 0,
        Err(_) => false,
    }
}