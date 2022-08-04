use std::hash::Hash;

#[derive(Debug, PartialEq)]
pub struct VecKeylist<K, V>(pub Vec<(K, V)>);

impl<K, V> VecKeylist<K, V> {
    pub fn new() -> Self {
        VecKeylist(Vec::new())
    }

    pub fn into_swapped(self) -> VecKeylist<V, K> {
        VecKeylist(self.0.into_iter().map(|(k, v)| (v, k)).collect())
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

impl<K: PartialEq, V> VecKeylist<K, V> {
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

impl<K, V> From<Vec<(K, V)>> for VecKeylist<K, V> {
    fn from(list: Vec<(K, V)>) -> Self {
        VecKeylist(list)
    }
}

impl<K: Clone, V: Clone> Clone for VecKeylist<K, V> {
    fn clone(&self) -> Self {
        VecKeylist(self.0.clone())
    }
}

impl<K: std::cmp::Ord, V> VecKeylist<K, V> {
    pub fn sort_by_key<'a>(&'a mut self) {
        self.0.sort_by(|a, b| a.0.cmp(&b.0))
    }
}

impl<K, V: std::cmp::Ord> VecKeylist<K, V> {
    pub fn sort_by_value<'a>(&'a mut self) {
        self.0.sort_by(|a, b| a.1.cmp(&b.1))
    }
}

impl<K: std::cmp::PartialEq, V: std::cmp::PartialEq> VecKeylist<K, V> {
    pub fn contains(&self, item: &(K, V)) -> bool {
        self.0.contains(item)
    }
}

impl<K: std::cmp::Ord, V: std::cmp::Ord> VecKeylist<K, V> {
    pub fn sort(&mut self) {
        self.0.sort()
    }

    /// The normal get function uses a find on a iterator to find the key value.
    /// This function uses binary search to find the key value
    pub fn get_key_value_sorted(&self, key: &K) -> Option<&(K, V)> {
        let index = self.0.binary_search_by_key(&key, |(a, _)| a).ok()?;
        self.0.get(index)
    }

    /// The normal get function uses a find on a iterator to find the value.
    /// This function uses binary search to find the value
    pub fn get_sorted(&self, key: &K) -> Option<&V> {
        let (_, v) = self.get_key_value_sorted(key)?;
        Some(v)
    }
}

use std::vec::IntoIter;

impl<K, V> IntoIterator for VecKeylist<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<K, V> Extend<(K, V)> for VecKeylist<K, V> {
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}

impl<'a, K: Copy, V: Copy> Extend<(&'a K, &'a V)> for VecKeylist<K, V> {
    fn extend<T: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: T) {
        self.0.extend(iter.into_iter().map(|(k, v)| (*k, *v)))
    }
}

impl<K, V> std::iter::FromIterator<(K, V)> for VecKeylist<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> VecKeylist<K, V> {
        VecKeylist(iter.into_iter().collect())
    }
}

impl<K: Hash, V: Hash> Hash for VecKeylist<K, V> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[cfg(feature = "serde")]
mod serde {
    use crate::VecKeylist;
    use serde::de::{Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
    use serde::ser::{Serialize, Serializer};
    use std::marker::PhantomData;

    impl<K: Serialize, V: Serialize> Serialize for VecKeylist<K, V> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_newtype_struct("VecKeylist", &self.0)
        }
    }

    struct KeylistVisitor<K, V> {
        marker: PhantomData<fn() -> VecKeylist<K, V>>,
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
        K: Deserialize<'de>,
        V: Deserialize<'de>,
    {
        type Value = VecKeylist<K, V>;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("Struct VecKeylist")
        }

        fn visit_seq<X>(self, mut seq: X) -> Result<Self::Value, X::Error>
        where
            X: SeqAccess<'de>,
        {
            let mut buffer = Vec::new();

            while let Some(x) = seq.next_element()? {
                buffer.push(x)
            }
            Ok(VecKeylist(buffer))
        }

        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut buffer = Vec::new();

            while let Some(x) = access.next_entry()? {
                buffer.push(x)
            }
            Ok(VecKeylist(buffer))
        }

        fn visit_newtype_struct<D>(self, deserializer: D) -> Result<VecKeylist<K, V>, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_newtype_struct("VecKeylist", KeylistVisitor::new())
        }
    }

    impl<'de, K, V> Deserialize<'de> for VecKeylist<K, V>
    where
        K: Deserialize<'de>,
        V: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any(KeylistVisitor::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::VecKeylist;
    use std::iter::FromIterator;

    #[test]
    fn from_into_iterator() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        map.insert("testing", 1);

        let keylist = VecKeylist::from_iter(map);

        let expected = VecKeylist(vec![("testing", 1)]);

        assert_eq!(keylist, expected);
    }

    #[test]
    fn sort() {
        let mut map = Vec::new();
        map.push(("a", 4));
        map.push(("c", 3));
        map.push(("b", 2));
        map.push(("d", 1));

        let mut keylist = VecKeylist::from_iter(map);
        keylist.sort();

        let expected = VecKeylist(vec![("a", 4), ("b", 2), ("c", 3), ("d", 1)]);

        assert_eq!(keylist, expected);
    }

    #[test]
    fn into_swapped() {
        let expected = VecKeylist(vec![(4, "a"), (3, "c")]);
        let mut map = Vec::new();
        map.push(("a", 4));
        map.push(("c", 3));

        let keylist = VecKeylist::from_iter(map).into_swapped();

        assert_eq!(expected, keylist);
    }

    #[test]
    fn get() {
        let keylist = VecKeylist(vec![("a", 4), ("a", 9), ("b", 2), ("c", 3), ("d", 1)]);

        assert_eq!(keylist.get(&"a"), Some(&4));
        assert_eq!(keylist.get(&"d"), Some(&1));
        assert_eq!(keylist.get(&"z"), None);
    }

    #[test]
    fn get_all() {
        let keylist = VecKeylist(vec![("a", 4), ("a", 9), ("b", 2), ("c", 3), ("d", 1)]);

        assert_eq!(keylist.get_all(&"a"), vec![&4, &9]);
        assert_eq!(keylist.get_all(&"z"), Vec::<&u32>::new());
        assert_eq!(keylist.get_all(&"d"), vec![&1]);
    }

    #[test]
    fn into_iter() {
        let keylist = VecKeylist(vec![("a", 4), ("b", 2)]);

        let mut items = keylist.into_iter();

        assert_eq!(Some(("a", 4)), items.next());
        assert_eq!(Some(("b", 2)), items.next());
    }

    #[test]
    fn get_sorted() {
        let keylist = VecKeylist(vec![("a", 4), ("a", 9), ("b", 2), ("c", 3), ("d", 1)]);

        assert_eq!(keylist.get_sorted(&"b"), Some(&2));
        assert_eq!(keylist.get_sorted(&"f"), None);
    }

    #[test]
    fn hash() {
        use std::hash::{Hash, Hasher};

        let mut hasher = std::collections::hash_map::DefaultHasher::default();

        let keylist = VecKeylist(vec![("a", 4), ("a", 9), ("b", 2), ("c", 3), ("d", 1)]);

        keylist.hash(&mut hasher);
        assert_eq!(145292038701700647, hasher.finish())
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use crate::VecKeylist;
    use serde_test::{assert_de_tokens, assert_ser_tokens, assert_tokens, Token};

    #[test]
    fn serde_de_list() {
        let expected = VecKeylist(vec![("oke", 1), ("test", 15)]);

        assert_de_tokens(
            &expected,
            &[
                Token::Seq { len: Some(2) },
                Token::Tuple { len: 2 },
                Token::BorrowedStr("oke"),
                Token::I32(1),
                Token::TupleEnd,
                Token::Tuple { len: 2 },
                Token::BorrowedStr("test"),
                Token::I32(15),
                Token::TupleEnd,
                Token::SeqEnd,
            ],
        );
    }

    #[test]
    fn serde_de_map() {
        let expected = VecKeylist(vec![("oke", 1), ("test", 15)]);

        assert_de_tokens(
            &expected,
            &[
                Token::Map { len: Some(2) },
                Token::BorrowedStr("oke"),
                Token::I32(1),
                Token::BorrowedStr("test"),
                Token::I32(15),
                Token::MapEnd,
            ],
        );
    }

    #[test]
    fn serde_ser() {
        let input = VecKeylist(vec![("oke", 1), ("test", 15)]);

        assert_ser_tokens(
            &input,
            &[
                Token::NewtypeStruct { name: "VecKeylist" },
                Token::Seq { len: Some(2) },
                Token::Tuple { len: 2 },
                Token::Str("oke"),
                Token::I32(1),
                Token::TupleEnd,
                Token::Tuple { len: 2 },
                Token::Str("test"),
                Token::I32(15),
                Token::TupleEnd,
                Token::SeqEnd,
            ],
        );
    }

    #[test]
    fn serde_round_trip() {
        let input = VecKeylist(vec![("oke", 1), ("test", 15)]);

        assert_tokens(
            &input,
            &[
                Token::NewtypeStruct { name: "VecKeylist" },
                Token::Seq { len: Some(2) },
                Token::Tuple { len: 2 },
                Token::BorrowedStr("oke"),
                Token::I32(1),
                Token::TupleEnd,
                Token::Tuple { len: 2 },
                Token::BorrowedStr("test"),
                Token::I32(15),
                Token::TupleEnd,
                Token::SeqEnd,
            ],
        );
    }
}
