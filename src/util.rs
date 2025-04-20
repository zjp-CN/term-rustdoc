pub use compact_str::{
    format_compact as xformat, CompactString as XString, CompactStringExt, ToCompactString,
};

pub use rustc_hash::FxHashMap as HashMap;

/// Construct a [`rustc_hash::FxHashMap`].
pub fn hashmap<K, V>(cap: usize) -> HashMap<K, V> {
    HashMap::with_capacity_and_hasher(cap, Default::default())
}

/// Join a vec of string by `::`.
pub fn join_path(path: &[String]) -> XString {
    path.iter().map(|path| path.as_str()).join_compact("::")
}
