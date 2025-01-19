pub struct Matrix(Vec<Vec<u32>>);

impl Matrix {
    pub fn new(input: &str) -> Self {
        Self(
            input
                .lines()
                .collect::<Vec<&str>>()
                .iter()
                .map(|l| {
                    l.split(" ")
                        .map(|c| c.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<Vec<u32>>>(),
        )
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        if row_no > self.0.len() {
            return None;
        }

        Some(self.0[row_no - 1].clone())
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        (col_no >= 1 && col_no <= self.0[0].len())
            .then(|| self.0.iter().map(|row| row[col_no - 1]).collect())
    }
}
