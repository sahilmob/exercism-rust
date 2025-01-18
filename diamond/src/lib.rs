pub fn get_diamond(c: char) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let mut char_idx = 0;
    for (i, ch) in ('A'..='Z').into_iter().enumerate() {
        if c == ch {
            char_idx = i;
            break;
        }
    }

    let side = (char_idx * 2) + 1;
    let iter_end = if side % 2 == 0 {
        side / 2 - 1
    } else {
        side / 2
    };

    for i in 0..=iter_end {
        let mut result_string = String::new();
        for j in 0..=iter_end * 2 {
            if i == 0 && j == iter_end {
                result_string.push(char::from((i + 65) as u8));
            } else if j == iter_end - i || j == iter_end + i {
                result_string.push(char::from((i + 65) as u8));
            } else {
                result_string.push(' ');
            }
        }
        result.push(result_string);
    }

    let mut result_rev = result.clone();

    result_rev.reverse();

    result.append(&mut result_rev[1..].to_vec());

    result
}
