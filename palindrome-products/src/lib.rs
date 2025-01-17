/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        let is_palindrome = is_palindrome(value);

        if is_palindrome || value == u64::MIN || value == u64::MAX {
            return Some(Self(value));
        }
        None
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

fn is_palindrome(value: u64) -> bool {
    let num_str = value.to_string();
    num_str == num_str.chars().rev().collect::<String>()
    // let mut reverse = 0;
    // let mut r = value;
    // while r > 0 {
    //     reverse = reverse * 10 + r % 10;
    //     r /= 10;
    // }
    // value == reverse
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut smallest = None;
    let mut largest = None;
    for i in min..=max {
        for j in min..=max {
            let product = i * j;
            if is_palindrome(product) {
                if product < smallest.unwrap_or(u64::MAX) {
                    smallest = Some(product)
                }
                if product > largest.unwrap_or(u64::MIN) {
                    largest = Some(product)
                }
            }
        }
    }

    match (smallest, largest) {
        (Some(s), Some(l)) => Some((Palindrome(s), Palindrome(l))),
        _ => None,
    }
}
