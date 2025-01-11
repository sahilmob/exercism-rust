pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|x: char| x == '-' || x.is_whitespace())
        .filter(|v| !v.is_empty())
        .map(|w| {
            let mut prev_char = 'a';
            let mut result = String::new();

            w.chars().for_each(|c| {
                if result.len() == 0 && c.is_alphabetic()
                    || c.is_uppercase() && !prev_char.is_uppercase()
                {
                    result.push(c.to_ascii_uppercase());
                }
                prev_char = c;
            });

            result
        })
        .collect::<String>()
}
