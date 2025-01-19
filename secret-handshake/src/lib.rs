pub fn actions(n: u8) -> Vec<&'static str> {
    let actions: Vec<(u8, &'static str)> = Vec::from([
        (1, "wink"),
        (2, "double blink"),
        (4, "close your eyes"),
        (8, "jump"),
        (16, "reverse"),
    ]);

    let mut reverse = false;
    let mut result: Vec<&'static str> = Vec::new();

    for (b, a) in actions {
        println!("{a}");
        if n & b > 0 {
            if a != "reverse" {
                result.push(a);
            } else {
                reverse = true;
            }
        }
    }

    if reverse {
        result.reverse();
    }

    result
}
