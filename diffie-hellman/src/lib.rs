use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    let mut primes: Vec<u64> = Vec::new();

    let _ = (2..=rng.gen_range(2..p)).filter(|x| {
        if !primes.iter().any(|n| x % n == 0) {
            primes.push(*x);
            return true;
        }
        return false;
    });

    if primes.last().is_some() {
        *primes.last().unwrap()
    } else {
        2
    }
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    let mut c = 1;
    let mut e_prime = 0;

    while e_prime < a {
        e_prime += 1;
        c = (c * g) % p
    }

    c
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    let mut c = 1;
    let mut e_prime = 0;

    while e_prime < a {
        e_prime += 1;
        c = (c * b_pub) % p
    }

    c
}
