use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    let mut primes: Vec<u64> = Vec::new();

    let _ = (2..=rng.gen_range(2..p)).filter(|x| {
        if !primes.iter().any(|n| x % n == 0) {
            primes.push(*x);
            true
        } else {
            false
        }
    });

    if primes.last().is_some() {
        *primes.last().unwrap()
    } else {
        2
    }
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    let mut e = a;
    let mut b = g;
    let mut result = 1;

    while e > 0 {
        if e % 2 == 1 {
            result = (result * b) % p;
        }
        b = (b * b) % p;
        e /= 2;
    }

    result
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    public_key(p, b_pub, a)
}
