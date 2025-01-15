use std::ops::Add;

pub struct Triangle<T>
where
    T: Copy + Add<Output = T> + PartialOrd + From<i32>,
{
    sides: [T; 3],
}

impl<T> Triangle<T>
where
    T: Copy + Add<Output = T> + PartialOrd + From<i32>,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let [s1, s2, s3] = sides;
        if !sides.iter().all(|n| n > &T::from(0)) {
            None
        } else if s1 + s2 < s3 || s1 + s3 < s2 || s2 + s3 < s1 {
            None
        } else {
            Some(Triangle { sides })
        }
    }

    pub fn is_equilateral(&self) -> bool {
        let [s1, s2, s3] = self.sides;
        s1 == s2 && s2 == s3
    }

    pub fn is_scalene(&self) -> bool {
        let [s1, s2, s3] = self.sides;
        s1 != s2 && s1 != s3 && s2 != s3
    }

    pub fn is_isosceles(&self) -> bool {
        let [s1, s2, s3] = self.sides;
        if s1 == s2 || s1 == s3 || s2 == s3 {
            true
        } else {
            false
        }
    }
}
