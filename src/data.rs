use std::collections::HashMap;
use std::hash::Hash;
use std::iter::Iterator;

pub fn morloc_pack_map<K, V>(xs: &(Vec<K>, Vec<V>)) -> HashMap<K, V>
where
    K: Clone + Eq + Hash + PartialEq,
    V: Clone,
{
    Iterator::zip(xs.0.iter(), xs.1.iter())
        .map(|v| (v.0.clone(), v.1.clone()))
        .collect()
}

pub fn morloc_unpack_map<K, V>(xs: &HashMap<K, V>) -> (Vec<K>, Vec<V>)
where
    K: Clone + Eq + Hash + PartialEq,
    V: Clone,
{
    xs.iter().map(|v| (v.0.clone(), v.1.clone())).unzip()
}
