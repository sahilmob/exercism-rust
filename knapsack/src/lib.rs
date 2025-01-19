#[derive(Clone)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}
pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let mut max_value = 0;
    let mut items = items.to_vec();
    while let Some(item) = items.pop() {
        let val = if item.weight <= max_weight {
            item.value + maximum_value(max_weight - item.weight, &items)
        } else {
            0
        };
        max_value = if val > max_value { val } else { max_value }
    }
    max_value
}
