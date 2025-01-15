/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.to_lowercase().chars().fold(0, |acc, curr| {
        acc + match curr {
            'k' => 5,
            n if n == 'd' || n == 'g' => 2,
            n if n == 'b' || n == 'c' || n == 'm' || n == 'p' => 3,
            n if n == 'f' || n == 'h' || n == 'v' || n == 'w' || n == 'y' => 4,
            n if n == 'j' || n == 'x' => 8,
            n if n == 'q' || n == 'z' => 10,
            n if n == 'a'
                || n == 'e'
                || n == 'i'
                || n == 'o'
                || n == 'u'
                || n == 'l'
                || n == 'n'
                || n == 'r'
                || n == 's'
                || n == 't' =>
            {
                1
            }

            _ => 0,
        }
    })
}
