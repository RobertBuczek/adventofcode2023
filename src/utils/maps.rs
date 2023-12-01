use std::collections::HashMap;
use std::hash::Hash;

pub fn find_key_for_value<K, V>(map: &HashMap<K, V>, value: V) -> Option<K>
where
    K: Eq + Hash + Clone,
    V: Eq,
{
    map.iter().find_map(|(key, val)| {
        if *val == value {
            Some(key.clone())
        } else {
            None
        }
    })
}
