pub fn egg_count(display_value: u32) -> usize {
    (0..32).rev().map(|n| (display_value >> n) & 1).sum::<u32>() as usize
}
