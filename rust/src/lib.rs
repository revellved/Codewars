pub mod num_get_index;

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

#[macro_export]
macro_rules! map_key_and_expr {
    () => { HashMap::new() };
    ($($key:ident : $val:expr),+) => {{
        let mut map = $crate::HashMap::new();
        $( map.insert(stringify!($key), $val); )*
        map
    }};
}

#[macro_export]
macro_rules! map_ss {
    () => { HashMap::new() };
    ($($key:literal : $value:literal),+) => {{
        let mut map = HashMap::new();
        $( map.insert(String::from_str($key).unwrap(), String::from_str($value).unwrap()); )*
        map
    }};
}

// pub struct ScopeCall<F: FnMut()> {
//     c: F,
// }
//
// impl<F: FnMut()> Drop for ScopeCall<F> {
//     fn drop(&mut self) {
//         (self.c)();
//     }
// }
//
// #[macro_export]
// macro_rules! defer {
//     ($e:expr) => {
//         let _scope_call = $crate::ScopeCall {
//             c: || -> () {
//                 $e;
//             },
//         };
//     };
// }
