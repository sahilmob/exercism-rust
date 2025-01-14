#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    strand: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    strand: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (i, c) in dna.chars().enumerate() {
            if c != 'A' && c != 'C' && c != 'G' && c != 'T' {
                return Err(i);
            }
        }

        Ok(Self {
            strand: dna.to_string(),
        })
    }

    pub fn into_rna(self) -> Rna {
        Rna::new(&self.strand.chars().fold(String::new(), |mut acc, char| {
            match char {
                'A' => acc.push('U'),
                'C' => acc.push('G'),
                'G' => acc.push('C'),
                _ => acc.push('A'),
            };
            acc
        }))
        .unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (i, c) in rna.chars().enumerate() {
            if c != 'A' && c != 'C' && c != 'G' && c != 'U' {
                return Err(i);
            }
        }

        Ok(Self {
            strand: rna.to_string(),
        })
    }
}
