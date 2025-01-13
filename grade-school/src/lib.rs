use std::collections::{HashMap, HashSet};

pub struct School<'a> {
    students: HashSet<&'a str>,
    grades: HashMap<u32, Vec<String>>,
}

impl<'a> School<'a> {
    pub fn new() -> School<'a> {
        Self {
            grades: HashMap::new(),
            students: HashSet::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &'a str) -> Result<(), &str> {
        self.grades.entry(grade).or_default();

        if self.students.contains(student) {
            return Err("Student exists!");
        }

        self.students.insert(student);

        self.grades
            .get_mut(&grade)
            .unwrap()
            .push(student.to_string());

        Ok(())
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut result: Vec<u32> = self.grades.keys().copied().collect();

        result.sort();

        result
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let f = self.grades.get(&grade);

        match f {
            Some(v) => {
                let mut result = v.clone();
                result.sort();
                result
            }
            None => Vec::new(),
        }
    }
}
