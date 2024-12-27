use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();
    let mut sorted_word_vec: Vec<char> = word.to_lowercase().chars().collect();
    sorted_word_vec.sort();
    let sorted_word: String = sorted_word_vec.into_iter().collect();

    for a in possible_anagrams {
        if a.to_lowercase() == word.to_lowercase() {
            continue;
        }

        let mut a_chars: Vec<char> = a.to_lowercase().chars().collect();
        a_chars.sort();
        let s = String::from_iter(a_chars);
        if s.to_lowercase() == sorted_word.to_lowercase() {
            result.insert(a.to_owned());
        }
    }

    result
}
