const STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let cup_idx = STUDENTS.iter().position(|n| *n == student).unwrap() * 2;

    diagram
        .lines()
        .flat_map(|l| {
            l[cup_idx..=cup_idx + 1].chars().map(|c| match c {
                'C' => "clover",
                'G' => "grass",
                'R' => "radishes",
                _ => "violets",
            })
        })
        .collect()
}
