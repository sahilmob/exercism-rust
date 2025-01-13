use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result: BTreeMap<char, i32> = BTreeMap::new();

    h.iter().for_each(|(score, chars)| {
        let mut chars = chars.clone();
        chars.sort();
        chars.iter().for_each(|c| {
            result.insert(c.to_ascii_lowercase(), *score);
        });
    });

    result
}
