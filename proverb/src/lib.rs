pub fn build_proverb(list: &[&str]) -> String {
    let mut result = String::from("");

    if list.len() == 0 {
        return result;
    }

    let final_str = format!("And all for the want of a {}.", list[0]);

    (0..list.len()).for_each(|i| {
        if i < list.len() - 1 {
            result.push_str(
                format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]).as_str(),
            );
        }
    });

    result.push_str(&final_str);

    result
}
