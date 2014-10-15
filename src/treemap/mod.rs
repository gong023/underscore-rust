use std::collections::TreeMap;

pub mod expands;

/// UnderscoreTreeMap expands TreeMap
/// usage:
///
/// ```
/// pub use __::vec::UnderscoreTreeMap;
/// ```
pub trait UnderscoreTreeMap<K, V> {
    fn invert(self) -> TreeMap<V, K>;

    fn pick(self, keys: &Vec<K>) -> TreeMap<K, V>;

    fn pick_by_filter(self, f: |k: &K, v: &V| -> bool) -> TreeMap<K, V>;

    fn omit(self, keys: &Vec<K>) -> TreeMap<K, V>;

    fn omit_by_filter(self, f: |k: &K, v: &V| -> bool) -> TreeMap<K, V>;

    fn defaults(self, appends: TreeMap<K, V>) -> TreeMap<K, V>;

//    fn pairs()
}
