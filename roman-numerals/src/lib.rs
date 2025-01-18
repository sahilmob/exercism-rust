const NUMERALS: [(usize, [&'static str; 10]); 4] = [
    (
        1000,
        ["", "M", "MM", "MMM", "--", "-", "--", "---", "----", "--"],
    ),
    (
        100,
        ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"],
    ),
    (
        10,
        ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"],
    ),
    (
        1,
        ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"],
    ),
];
pub struct Roman;
impl Roman {
    pub fn from(n: usize) -> String {
        if n > 3999 {
            return "".to_string();
        }
        NUMERALS
            .iter()
            .map(|&(base, nums)| nums[(n / base) % 10])
            .collect()
    }
}
