
//! macro for creating linked_hash_maps
//! This macro is essentially the same as the one found in the maplit
//! crate; it has simply been altered to create LinkedHashMaps instead
//! of HashMaps
extern crate linked_hash_map;

pub fn s(input: &str) -> String {
    String::from(input)
}

#[macro_export]
/// Create a **LinkedHashMap** from a list of key-value pairs
///
/// ## Example
///
/// ```ignore
/// #[macro_use] extern crate linkedhashmap-macro;
/// # fn main() {
///
/// let map = linkedhashmap!{
///     "a" => 1,
///     "b" => 2,
/// };
/// assert_eq!(map["a"], 1);
/// assert_eq!(map["b"], 2);
/// assert_eq!(map.get("c"), None);
/// # }
/// ```
macro_rules! linkedhashmap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(linkedhashmap!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { linkedhashmap!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = linkedhashmap!(@count $($key),*);
            let mut _map = linked_hash_map::LinkedHashMap::with_capacity(_cap);
            $(
                let _ = _map.insert($key, $value);
            )*
            _map
        }
    };
}


#[cfg(test)]
mod test {
    use super::*;
    use linked_hash_map::LinkedHashMap;

    #[test]
    fn linkedhashmap() {
        let hm: LinkedHashMap<String,String> = linkedhashmap!(s("a") =>s("b") );
        let mut expected = LinkedHashMap::new();
        expected.insert(s("a"),s("b"));
        assert_eq!(hm, expected);
    }
}