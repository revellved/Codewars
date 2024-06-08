pub use std::collections::HashMap;

#[macro_export]
macro_rules! map_key_val {
    () => { HashMap::new() };
    ($($key:literal : $val:expr),+) => {{
        let mut map = $crate::HashMap::new();
        $( map.insert($key, $val); )*
        map
    }};
}
