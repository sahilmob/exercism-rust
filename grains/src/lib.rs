pub fn square(s: u32) -> u64 {
    if s == 1 {
        1
    } else {
        2 * square(s - 1)
    }
}

pub fn total() -> u64 {
    (1..=64).map(square).sum()
}
