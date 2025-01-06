pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut result: Vec<u64> = Vec::new();
    (2..=n).for_each(|v| {
        if n % v == 0 {
            while n % v == 0 {
                result.push(v);
                n = n / v
            }
        }
    });

    result
}
