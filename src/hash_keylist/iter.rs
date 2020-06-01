use crate::HashKeylist;
use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};

impl<'a, K, V, S> IntoIterator for HashKeylist<K, V, S>
where
    K: Hash + Eq,
    V: Eq,
    S: BuildHasher,
{
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            keys: self.keys.into_iter(),
            map: self
                .data
                .into_iter()
                .map(|(k, vs)| (k, vs.into_iter()))
                .collect(),
        }
    }
}

pub struct IntoIter<K, V> {
    pub(crate) keys: std::vec::IntoIter<K>,
    pub(crate) map: HashMap<K, std::vec::IntoIter<V>>,
}

impl<K, V> Iterator for IntoIter<K, V>
where
    K: Hash + Eq,
    V: Eq,
{
    type Item = (K, V);
    fn next(&mut self) -> Option<Self::Item> {
        let key = self.keys.next()?;
        let value = self.map.get_mut(&key)?.next()?;
        Some((key, value))
    }
}

pub struct IterMut<'a, K, V> {
    pub(crate) keys: std::slice::Iter<'a, K>,
    pub(crate) map: HashMap<&'a K, RowIterMut<'a, V>>,
}

impl<'a, K, V> Iterator for IterMut<'a, K, V>
where
    K: Hash + Eq,
    V: Eq,
{
    type Item = (&'a K, &'a mut V);
    fn next(&mut self) -> Option<Self::Item> {
        let key = self.keys.next()?;
        let value = self.map.get_mut(key)?.next()?;
        Some((key, value))
    }
}

pub struct RowIterMut<'a, V> {
    pub(crate) values: std::slice::IterMut<'a, V>,
}

impl<'a, V> Iterator for RowIterMut<'a, V> {
    type Item = &'a mut V;
    fn next(&mut self) -> Option<Self::Item> {
        self.values.next()
    }
}

pub struct Iter<'a, K, V> {
    pub(crate) keys: std::slice::Iter<'a, K>,
    pub(crate) map: HashMap<&'a K, RowIter<'a, V>>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V>
where
    K: Hash + Eq,
    V: Eq,
{
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        let key = self.keys.next()?;
        let value = self.map.get_mut(key)?.next()?;
        Some((key, value))
    }
}

pub struct RowIter<'a, V> {
    pub(crate) values: std::slice::Iter<'a, V>,
}

impl<'a, V> Iterator for RowIter<'a, V> {
    type Item = &'a V;
    fn next(&mut self) -> Option<Self::Item> {
        self.values.next()
    }
}

pub struct IterKeyValue<'a, K, V> {
    pub(crate) key: &'a K,
    pub(crate) values: std::slice::Iter<'a, V>,
}

impl<'a, K, V> Iterator for IterKeyValue<'a, K, V> {
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        Some((self.key, self.values.next()?))
    }
}
