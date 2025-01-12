use self::Allergen::*;
use std::slice::Iter;

pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergen {
    pub fn iterator() -> Iter<'static, Allergen> {
        static ALLERGEN: [Allergen; 8] = [
            Eggs,
            Peanuts,
            Shellfish,
            Strawberries,
            Tomatoes,
            Chocolate,
            Pollen,
            Cats,
        ];
        ALLERGEN.iter()
    }

    fn clone(&self) -> Allergen {
        match self {
            Eggs => Eggs,
            Peanuts => Peanuts,
            Shellfish => Shellfish,
            Strawberries => Strawberries,
            Tomatoes => Tomatoes,
            Chocolate => Chocolate,
            Pollen => Pollen,
            Cats => Cats,
        }
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        match self.score {
            0 => false,
            n => {
                let mut i = 7;
                let mut score = if n > 255 { (n as i32) % 256 } else { n as i32 };
                for a in Allergen::iterator().rev() {
                    let a_score = 2_i32.pow(i);
                    let diff = score - a_score;

                    if a_score > score {
                        i -= 1;
                        continue;
                    }

                    if a == allergen {
                        return true;
                    }

                    if diff <= 0 {
                        return false;
                    }

                    score = diff;

                    i -= 1
                }

                false
            }
        }
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut result: Vec<Allergen> = Vec::new();

        Allergen::iterator().for_each(|a| {
            if self.is_allergic_to(a) {
                result.push(a.clone());
            }
        });

        result
    }
}
