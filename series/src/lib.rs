pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len > digits.len() {
        return Vec::new();
    }

    digits
        .to_string()
        .split("")
        .filter(|s| !s.is_empty())
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .windows(len)
        .map(|w| w.join(""))
        .collect()
}
