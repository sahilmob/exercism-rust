#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores().iter().max().cloned()
    }

    pub fn personal_top_three(self) -> Vec<u32> {
        let mut x = self.scores.to_vec();
        x.sort_by(|a, b| b.partial_cmp(a).unwrap());

        x.truncate(3);

        x
    }
}
