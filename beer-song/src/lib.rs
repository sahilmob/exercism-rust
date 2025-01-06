pub fn verse(n: u32) -> String {
    let first_part = if n == 1 {
        format!("{n} bottle of beer on the wall")
    } else if n > 0 {
        format!("{n} bottles of beer on the wall")
    } else {
        "No more bottles of beer on the wall".to_string()
    };
    let second_part = if n == 1 {
        format!("{n} bottle of beer")
    } else if n > 0 {
        format!("{n} bottles of beer")
    } else {
        "no more bottles of beer".to_string()
    };

    let third_part = if n == 1 {
        "Take it down and pass it around".to_string()
    } else if n > 0 {
        "Take one down and pass it around".to_string()
    } else {
        "Go to the store and buy some more".to_string()
    };

    let fourth_part = if n == 2 {
        format!("{} bottle of beer on the wall", n - 1)
    } else if n > 1 {
        format!("{} bottles of beer on the wall", n - 1)
    } else if n == 1 {
        "no more bottles of beer on the wall".to_string()
    } else {
        "99 bottles of beer on the wall".to_string()
    };
    format!(
        "{}, {}.\n{}, {}.",
        first_part, second_part, third_part, fourth_part
    )
}

pub fn sing(start: u32, end: u32) -> String {
    (end..start + 1)
        .rev()
        .enumerate()
        .map(|(i, n)| {
            if i != 0 {
                String::from("\n\n") + verse(n).as_str()
            } else {
                verse(n)
            }
        })
        .collect::<String>()
}
