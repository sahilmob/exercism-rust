#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    match () {
        _ if from_base < 2 => return Err(Error::InvalidInputBase),
        _ if to_base < 2 => return Err(Error::InvalidOutputBase),
        _ => {}
    }
    if let Some(&invalid_digit) = number.iter().find(|&&n| n >= from_base) {
        return Err(Error::InvalidDigit(invalid_digit));
    }

    let normalized_num = number
        .iter()
        .copied()
        .fold(0, |acc, next| acc * from_base + next);

    Ok(BaseBuilder::build(normalized_num, to_base))
}

struct BaseBuilder {
    base: u32,
    remainder: u32,
}

impl Iterator for BaseBuilder {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remainder == 0 {
            return None;
        }
        let n = self.remainder % self.base;
        self.remainder /= self.base;
        Some(n)
    }
}

impl BaseBuilder {
    fn build(number: u32, base: u32) -> Vec<u32> {
        if number == 0 {
            return vec![0];
        }
        let mut res: Vec<_> = Self {
            base,
            remainder: number,
        }
        .collect::<Vec<_>>();

        // convert to big endian for output
        res.reverse();

        res
    }
}
