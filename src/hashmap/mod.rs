use std::collections::HashMap;

pub mod expands;

/// UnderscoreHashMap expands HashMap
/// usage:
///
/// ```
/// pub use __::vec::UnderscoreHashMap;
/// ```
pub trait UnderscoreHashMap<K, V> {
    fn invert(self) -> HashMap<V, K>;

    fn pick(self, keys: Vec<K>) -> HashMap<K, V>;

    fn pick_by_filter(self, f: |k: &K, v: &V| -> bool) -> HashMap<K, V>;

    fn omit(self, keys: Vec<K>) -> HashMap<K, V>;

    fn omit_by_filter(self, f: |k: &K, v: &V| -> bool) -> HashMap<K, V>;

    fn defaults(self, appends: HashMap<K, V>) -> HashMap<K, V>;

    // needs #![feature(unboxed_closures, unboxed_closure_sugar)] and the are still experimental
    // fn property(self, key: K) -> Box<|&:|:'static -> V>;
}
