pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut i = 2;
    let mut result: Vec<u64> = Vec::new();
    while i <= n {
        while n % i == 0 {
            result.push(i);
            n /= i
        }

        i += 1
    }

    result
}
