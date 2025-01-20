#[macro_export]
macro_rules! hashmap {
    // Empty case
    () => {{
        use crate::HashMap;
        HashMap::new()
    }};

    // Single pair with optional trailing comma
    ($key:literal => $v:expr $(,)?) => {{
        use crate::HashMap;
        let mut hm = HashMap::new();
        hm.insert($key, $v);
        hm
    }};

    // Multiple pairs with required comma
    ($key:literal => $v:expr, $($rest:literal => $rv:expr),+ $(,)?) => {{
        use crate::HashMap;
        let mut hm = HashMap::new();
        hm.insert($key, $v);
        $(
            hm.insert($rest, $rv);
        )+
        hm
    }};
}
