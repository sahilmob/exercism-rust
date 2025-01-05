/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.len() < 2 {
        return false;
    }

    let mut sum = 0;
    let mut char_counter = 0;
    let rev_chars = code.chars().rev();

    for v in rev_chars {
        if v.is_whitespace() && char_counter % 2 == 0 {
            char_counter = 0;
        } else if v.is_numeric() {
            let mut parsed_v = v.to_string().parse::<i32>().unwrap();
            if char_counter % 2 != 0 {
                parsed_v *= 2;
                if parsed_v > 9 {
                    parsed_v -= 9
                }
            }

            sum += parsed_v;
            char_counter += 1
        } else if v.is_ascii() && !v.is_whitespace() {
            if sum % 10 == 0 {
                sum += 1;
            }
            break;
        }
    }

    (sum == 0 && code.trim().len() > 1) || (sum > 0 && sum % 10 == 0)
}
