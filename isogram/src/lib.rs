use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut chars: HashSet<String> = HashSet::new();

    for c in candidate.chars() {
        let str: String = c.to_lowercase().collect();
        if chars.contains(&str) {
            return false;
        }
        if c.is_alphabetic() {
            chars.insert(str);
        }
    }

    true
}
