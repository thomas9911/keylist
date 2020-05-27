//!
//! Map like wrapper around a list with tuple pairs. Inspired by Elixir's Keyword lists.
//!
//! Because it is just a list with tuples, keys can be anything that can be put in a tuple. Also means that getting a value is not efficient.
//!
//! For something more usefull and quite similar take a look at: https://docs.rs/multimap
//! ```
//! use keylist::Keylist;
//!
//! let mut keylist = Keylist::from(vec![("a", 5), ("b", 2), ("a", 1)]);
//!
//! assert_eq!(keylist.get(&"a"), Some(&5));
//!
//! keylist.sort_by_value();
//!
//! assert_eq!(keylist.get(&"a"), Some(&1));
//!
//! keylist.push("z", 26);
//!
//! assert_eq!(keylist.get(&"z"), Some(&26));
//!
//! keylist.insert(1, "z", 2);
//!
//! assert_eq!(keylist.get(&"z"), Some(&2));
//!
//! assert_eq!(keylist.get_all(&"z"), vec![&2, &26]);
//!
//! assert_eq!(keylist.get_key_value(&"b"), Some(&("b", 2)));
//!
//! let mut swapped_keylist = keylist.into_swapped();
//!
//! assert_eq!(swapped_keylist.get(&2), Some(&"z"));
//!
//! assert_eq!(swapped_keylist.get_all(&2), vec![&"z", &"b"]);
//!
//! swapped_keylist.sort();
//!
//! assert_eq!(swapped_keylist.get_all(&2), vec![&"b", &"z"]);
//!
//! swapped_keylist.extend(vec![(3, "b"), (2, "g")]);
//!
//! assert_eq!(swapped_keylist.get_all(&2), vec![&"b", &"z", &"g"]);
//!
//! ```
//!
//! Convert keylist to map and back:
//! ```
//! use std::collections::HashMap;
//! use std::iter::FromIterator;
//! use keylist::Keylist;
//!
//! let keylist = Keylist::from(vec![("a", 5), ("b", 2), ("c", 1)]);
//!
//! let mut map = HashMap::new();
//! map.extend(vec![("a", 5), ("b", 2), ("c", 1)]);
//!
//! let map_from_keylist = HashMap::from_iter(keylist.clone());
//! let mut keylist_from_map = Keylist::from_iter(map.clone());
//! keylist_from_map.sort();
//!
//! assert_eq!(map, map_from_keylist);
//! assert_eq!(keylist, keylist_from_map);
//! ```
//!
//! Arbitrary example:
//! ```
//! use keylist::Keylist;
//!
//! let mut keylist = Keylist::new();
//! keylist.push(vec![3.12, 0.12], "a");
//! keylist.push(vec![0.1235, 34.121551], "c");
//!
//! assert_eq!(keylist.get(&vec![0.1235, 34.121551]), Some(&"c"));
//!
//!
//! ```

#[derive(Debug, PartialEq)]
pub struct Keylist<K, V>(pub Vec<(K, V)>);

impl<K, V> Keylist<K, V> {
    pub fn new() -> Self {
        Keylist(Vec::new())
    }

    pub fn into_swapped(self) -> Keylist<V, K> {
        Keylist(self.0.into_iter().map(|(k, v)| (v, k)).collect())
    }

    pub fn insert(&mut self, index: usize, k: K, v: V) {
        self.0.insert(index, (k, v))
    }

    pub fn push(&mut self, k: K, v: V) {
        self.0.push((k, v))
    }

    pub fn pop(&mut self) -> Option<(K, V)> {
        self.0.pop()
    }

    pub fn remove(&mut self, index: usize) -> (K, V) {
        self.0.remove(index)
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a (K, V)> {
        self.0.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut (K, V)> {
        self.0.iter_mut()
    }

    pub fn keys<'a>(&'a self) -> impl Iterator<Item = &'a K> {
        self.iter().map(|(k, _)| k)
    }

    pub fn keys_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut K> {
        self.iter_mut().map(|(k, _)| k)
    }

    pub fn values<'a>(&'a self) -> impl Iterator<Item = &'a V> {
        self.iter().map(|(_, v)| v)
    }

    pub fn values_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut V> {
        self.iter_mut().map(|(_, v)| v)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<K: PartialEq, V> Keylist<K, V> {
    pub fn get_key_value(&self, key: &K) -> Option<&(K, V)> {
        self.iter().find(|x| &x.0 == key)
    }

    pub fn get_key_value_mut(&mut self, key: &K) -> Option<&mut (K, V)> {
        self.iter_mut().find(|x| &x.0 == key)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let (_, v) = self.get_key_value(key)?;
        Some(v)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let (_, v) = self.get_key_value_mut(key)?;
        Some(v)
    }

    pub fn get_all_get_key_value(&self, key: &K) -> Vec<&(K, V)> {
        self.iter().filter(|(k, _)| k == key).collect()
    }

    /// get all values matching the key
    pub fn get_all(&self, key: &K) -> Vec<&V> {
        self.iter()
            .filter(|(k, _)| k == key)
            .map(|(_, v)| v)
            .collect()
    }
}

impl<K, V> From<Vec<(K, V)>> for Keylist<K, V> {
    fn from(list: Vec<(K, V)>) -> Self {
        Keylist(list)
    }
}

impl<K: Clone, V: Clone> Clone for Keylist<K, V> {
    fn clone(&self) -> Self {
        Keylist(self.0.clone())
    }
}

impl<K: std::cmp::Ord, V> Keylist<K, V> {
    pub fn sort_by_key<'a>(&'a mut self) {
        self.0.sort_by(|a, b| a.0.cmp(&b.0))
    }
}

impl<K, V: std::cmp::Ord> Keylist<K, V> {
    pub fn sort_by_value<'a>(&'a mut self) {
        self.0.sort_by(|a, b| a.1.cmp(&b.1))
    }
}

impl<K: std::cmp::PartialEq, V: std::cmp::PartialEq> Keylist<K, V> {
    pub fn contains(&self, item: &(K, V)) -> bool {
        self.0.contains(item)
    }
}

impl<K: std::cmp::Ord, V: std::cmp::Ord> Keylist<K, V> {
    pub fn sort(&mut self) {
        self.0.sort()
    }

    /// The normal get function uses a find on a iterator to find the key value
    /// This function uses binary search to find the key value
    pub fn get_key_value_sorted(&self, key: &K) -> Option<&(K, V)> {
        let index = self.0.binary_search_by_key(&key, |(a, _)| a).ok()?;
        self.0.get(index)
    }

    /// The normal get function uses a find on a iterator to find the value
    /// This function uses binary search to find the value
    pub fn get_sorted(&self, key: &K) -> Option<&V> {
        let (_, v) = self.get_key_value_sorted(key)?;
        Some(v)
    }
}

use std::vec::IntoIter;

impl<K, V> IntoIterator for Keylist<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<K, V> Extend<(K, V)> for Keylist<K, V> {
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}

impl<'a, K: Copy, V: Copy> Extend<(&'a K, &'a V)> for Keylist<K, V> {
    fn extend<T: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: T) {
        self.0.extend(iter.into_iter().map(|(k, v)| (*k, *v)))
    }
}

impl<K, V> std::iter::FromIterator<(K, V)> for Keylist<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Keylist<K, V> {
        Keylist(iter.into_iter().collect())
    }
}

use serde::de::{Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::ser::{Serialize, Serializer};
use std::marker::PhantomData;

impl<K: Serialize, V: Serialize> Serialize for Keylist<K, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct("Keylist", &self.0)
    }
}

struct KeylistVisitor<K, V> {
    marker: PhantomData<fn() -> Keylist<K, V>>,
}

impl<K, V> KeylistVisitor<K, V> {
    fn new() -> Self {
        KeylistVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de, K, V> Visitor<'de> for KeylistVisitor<K, V>
where
    K: serde::de::Deserialize<'de>,
    V: serde::de::Deserialize<'de>,
{
    type Value = Keylist<K, V>;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Struct Keylist")
    }

    fn visit_seq<X>(self, mut seq: X) -> Result<Self::Value, X::Error>
    where
        X: SeqAccess<'de>,
    {
        let mut buffer = Vec::new();

        while let Some(x) = seq.next_element()? {
            buffer.push(x)
        }
        Ok(Keylist(buffer))
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut buffer = Vec::new();

        while let Some(x) = access.next_entry()? {
            buffer.push(x)
        }
        Ok(Keylist(buffer))
    }

    // fn visit_newtype_struct<D>(self, mut deserializer: D) -> Result<Keylist<K, V>, D::Error>
    // where D: Deserializer<'de>, {
    //     deserializer.deserialize_newtype_struct("Keylist", KeylistVisitor::new())
    // }
}

impl<'de, K, V> Deserialize<'de> for Keylist<K, V>
where
    K: serde::de::Deserialize<'de>,
    V: serde::de::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(KeylistVisitor::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::Keylist;
    use std::iter::FromIterator;

    #[test]
    fn from_into_iterator() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        map.insert("testing", 1);

        let keylist = Keylist::from_iter(map);

        let expected = Keylist(vec![("testing", 1)]);

        assert_eq!(keylist, expected);
    }

    #[test]
    fn sort() {
        let mut map = Vec::new();
        map.push(("a", 4));
        map.push(("c", 3));
        map.push(("b", 2));
        map.push(("d", 1));

        let mut keylist = Keylist::from_iter(map);
        keylist.sort();

        let expected = Keylist(vec![("a", 4), ("b", 2), ("c", 3), ("d", 1)]);

        assert_eq!(keylist, expected);
    }

    #[test]
    fn into_swapped() {
        let expected = Keylist(vec![(4, "a"), (3, "c")]);
        let mut map = Vec::new();
        map.push(("a", 4));
        map.push(("c", 3));

        let keylist = Keylist::from_iter(map).into_swapped();

        assert_eq!(expected, keylist);
    }

    #[test]
    fn get() {
        let keylist = Keylist(vec![("a", 4), ("a", 9), ("b", 2), ("c", 3), ("d", 1)]);

        assert_eq!(keylist.get(&"a"), Some(&4));
        assert_eq!(keylist.get(&"d"), Some(&1));
        assert_eq!(keylist.get(&"z"), None);
    }

    #[test]
    fn get_all() {
        let keylist = Keylist(vec![("a", 4), ("a", 9), ("b", 2), ("c", 3), ("d", 1)]);

        assert_eq!(keylist.get_all(&"a"), vec![&4, &9]);
        assert_eq!(keylist.get_all(&"z"), Vec::<&u32>::new());
        assert_eq!(keylist.get_all(&"d"), vec![&1]);
    }

    #[test]
    fn into_iter() {
        let keylist = Keylist(vec![("a", 4), ("b", 2)]);

        let mut items = keylist.into_iter();

        assert_eq!(Some(("a", 4)), items.next());
        assert_eq!(Some(("b", 2)), items.next());
    }

    #[test]
    fn get_sorted() {
        let keylist = Keylist(vec![("a", 4), ("a", 9), ("b", 2), ("c", 3), ("d", 1)]);

        assert_eq!(keylist.get_sorted(&"b"), Some(&2));
        assert_eq!(keylist.get_sorted(&"f"), None);
    }

    #[test]
    fn serde_de() {
        let expected = Keylist(vec![("oke", 1), ("test", 15)]);

        let input = r#"
            [
                ["oke", 1],
                ["test", 15]
            ]
        "#;

        let keylist: Keylist<&str, u32> = serde_json::from_str(input).unwrap();

        let input = r#"
        {
            "oke": 1,
            "test": 15
        }
        "#;

        let second_keylist: Keylist<&str, u32> = serde_json::from_str(input).unwrap();

        assert_eq!(expected, keylist);
        assert_eq!(expected, second_keylist);
    }

    #[test]
    fn serde_ser() {
        use serde_json::Value::*;
        // rewrite to use serde_test
        let input = Keylist(vec![("oke", 1), ("test", 15)]);
        let expected = Array(vec![
            Array(vec![String("oke".to_string()), Number(1.into())]),
            Array(vec![String("test".to_string()), Number(15.into())]),
        ]);
        let output = serde_json::to_value(&input).unwrap();

        assert_eq!(expected, output);
    }
}