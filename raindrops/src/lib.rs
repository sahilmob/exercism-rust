pub fn raindrops(n: u32) -> String {
    let mut result = String::new();

    if n % 3 == 0 {
        result.push_str(String::from("Pling").as_str())
    }

    if n % 5 == 0 {
        result.push_str(String::from("Plang").as_str())
    }

    if n % 7 == 0 {
        result.push_str(String::from("Plong").as_str())
    }

    if result.is_empty() {
        result.push_str(n.to_string().as_str())
    }

    result
}
