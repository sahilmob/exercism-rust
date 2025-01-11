const STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let student_idx = STUDENTS.iter().position(|n| *n == student).unwrap();

    let mut result: Vec<&str> = Vec::new();
    let start_idx = 2 * student_idx;
    let ent_idx = start_idx + 2;

    diagram
        .split("\n")
        .collect::<Vec<_>>()
        .iter()
        .for_each(|r| {
            r[start_idx..ent_idx].chars().for_each(|c| match c {
                'C' => result.push("clover"),
                'G' => result.push("grass"),
                'R' => result.push("radishes"),
                'V' => result.push("violets"),
                _ => (),
            });
        });

    result
}
