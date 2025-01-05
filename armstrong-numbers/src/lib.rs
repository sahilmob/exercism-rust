pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let len = num_str.len() as u32;

    num_str
        .chars()
        .fold(0, |acc, num| num.to_digit(10).unwrap().pow(len) + acc)
        == num
}
