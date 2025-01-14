pub fn encode(source: &str) -> String {
    let mut current_char = '1';
    let mut current_char_count = 0;
    let mut result = String::new();

    for v in source.chars().chain("1".chars()) {
        if current_char == '1' {
            current_char = v;
        }

        if v != current_char {
            if current_char_count > 1 {
                result.push_str(&format!("{current_char_count}{current_char}"));
            } else {
                result.push(current_char);
            }
            current_char = v;
            current_char_count = 1;
        } else {
            current_char_count += 1;
        }
    }

    result
}

pub fn decode(source: &str) -> String {
    let mut current_count = 0;
    let mut result = String::new();

    for v in source.chars() {
        if v.is_numeric() {
            current_count = current_count * 10 + v.to_digit(10).unwrap();
        } else if current_count == 0 {
            result.push(v);
        } else {
            result.push_str(&v.to_string().repeat(current_count as usize));
            current_count = 0;
        }
    }

    result
}
