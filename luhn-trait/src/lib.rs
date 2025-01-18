pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        println!("{}", self.to_string());
        self.to_string()
            .chars()
            .filter(|c| !c.is_whitespace())
            .rev()
            .try_fold((0, 0), |(sum, count), c| {
                c.to_digit(10)
                    .map(|n| if count % 2 != 0 { n * 2 } else { n })
                    .map(|n| if n > 9 { n - 9 } else { n })
                    .map(|n| (sum + n, count + 1))
            })
            .map_or(false, |(sum, count)| count != 0 && sum % 10 == 0)
    }
}
