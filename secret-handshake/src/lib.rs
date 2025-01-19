pub fn actions(n: u8) -> Vec<&'static str> {
    let mut reverse = false;

    let mut result: Vec<&'static str> = [
        (1, "wink"),
        (2, "double blink"),
        (4, "close your eyes"),
        (8, "jump"),
        (16, "reverse"),
    ]
    .to_vec()
    .iter()
    .filter(|(b, _)| n & b > 0)
    .filter(|(_, a)| {
        if a == &"reverse" {
            reverse = true;
            return false;
        }
        true
    })
    .map(|(_, a)| *a)
    .collect();

    if reverse {
        result.reverse();
    }

    result
}
