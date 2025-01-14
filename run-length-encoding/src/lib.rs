pub fn encode(source: &str) -> String {
    let mut result = String::new();

    if source.is_empty() {
        return result;
    }

    let mut current_char = '1';
    let mut current_char_count = 0;

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
    let mut result = String::new();

    if source.is_empty() {
        return result;
    }

    let mut current_count = String::new();

    for v in source.chars() {
        if v.is_numeric() {
            current_count.push(v);
        } else if current_count.is_empty() {
            result.push(v);
        } else {
            (0..current_count.parse::<i32>().unwrap()).for_each(|_| result.push(v));
            current_count = String::new();
        }
    }

    result
}
