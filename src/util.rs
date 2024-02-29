pub use compact_str::{
    format_compact as xformat, CompactString as XString, CompactStringExt, ToCompactString,
};

pub use rustc_hash::FxHashMap as HashMap;
use std::hash::BuildHasherDefault;

pub type Hasher = BuildHasherDefault<rustc_hash::FxHasher>;

/// Construct a [`rustc_hash::FxHashMap`].
pub fn hashmap<K, V>(cap: usize) -> HashMap<K, V> {
    HashMap::with_capacity_and_hasher(cap, BuildHasherDefault::default())
}
