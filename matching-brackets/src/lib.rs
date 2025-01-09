pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<u8> = Vec::new();
    let mut closing_set_iter = [41, 93, 125].iter();
    string.as_bytes().iter().for_each(|curr| match curr {
        40 => stack.push(41),
        91 => stack.push(93),
        123 => stack.push(125),
        n if stack.last().eq(&Some(n)) => {
            stack.pop();
        }
        n if closing_set_iter.any(|v| n == v) => stack.push(*n),
        _ => (),
    });

    stack.is_empty()
}
