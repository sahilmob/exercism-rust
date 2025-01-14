#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    };

    (1..num)
        .try_fold(0, |mut acc, curr| {
            if num % curr == 0 {
                acc += curr;
            }

            Ok::<u64, ()>(acc)
        })
        .map_or(None, |n| match num.cmp(&n) {
            std::cmp::Ordering::Equal => Some(Classification::Perfect),
            std::cmp::Ordering::Less => Some(Classification::Abundant),
            std::cmp::Ordering::Greater => Some(Classification::Deficient),
        })
}
