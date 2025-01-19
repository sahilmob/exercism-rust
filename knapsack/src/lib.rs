use std::collections::HashMap;

#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

fn calc_max_value(
    max_weight: u32,
    items: &[Item],
    idx: usize,
    cache: &mut HashMap<String, u32>,
) -> u32 {
    let cache_key = format!("{}:{}", max_weight, idx);

    if cache.get(&cache_key).is_some() {
        return *cache.get(&cache_key).unwrap();
    }

    if max_weight == 0 {
        cache.insert(cache_key, 0);
        return 0;
    }

    if idx >= items.len() {
        cache.insert(cache_key, 0);
        return 0;
    }

    let mut result = 0;
    let item = &items[idx];

    if item.weight <= max_weight {
        result = item.value + calc_max_value(max_weight - item.weight, items, idx + 1, cache);
        let local_max_without_this_item = calc_max_value(max_weight, items, idx + 1, cache);
        result = result.max(local_max_without_this_item)
    } else {
        result = result.max(calc_max_value(max_weight, items, idx + 1, cache))
    }

    cache.insert(cache_key, result);

    result
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let mut cache: HashMap<String, u32> = HashMap::new();

    calc_max_value(max_weight, items, 0, &mut cache)
}
