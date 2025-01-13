/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    isbn.chars()
        .fold((0, 0, true), |(c, s, v), curr| match curr {
            '-' => (c, s, v),
            'X' if c == 9 => (c + 1, (s + 10) % 11, v),
            '0'..='9' => (c + 1, (s + curr.to_digit(10).unwrap() * (10 - c)) % 11, v),
            _ => (c, s, false),
        })
        == (10, 0, true)
}
