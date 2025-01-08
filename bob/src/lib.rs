pub fn reply(message: &str) -> &str {
    if message.trim() == "" {
        return "Fine. Be that way!";
    }

    let bytes = message.trim().as_bytes();

    if bytes.last().unwrap() == &b'?' {
        if bytes[..bytes.len() - 1].iter().all(|b| *b < 97) && bytes.iter().any(|b| *b > 64) {
            return "Calm down, I know what I'm doing!";
        } else {
            return "Sure.";
        }
    } else if bytes.iter().all(|b| *b < 97) && bytes.iter().any(|b| *b > 64) {
        return "Whoa, chill out!";
    }

    "Whatever."
}
