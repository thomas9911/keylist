use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};
use std::iter::FromIterator;

pub mod iter;
use iter::*;

#[derive(Debug)]
pub struct HashKeylist<K, V, S>
where
    K: Hash + Eq,
    V: Eq,
    S: BuildHasher,
{
    data: HashMap<K, Vec<V>, S>,
    keys: Vec<K>,
}

// fn make_hash<K: Hash + ?Sized>(hash_builder: &impl BuildHasher, val: &K) -> u64 {
//     let mut state = hash_builder.build_hasher();
//     val.hash(&mut state);
//     state.finish()
// }

impl<K, V, S> PartialEq for HashKeylist<K, V, S>
where
    K: Hash + Eq,
    V: Eq,
    S: BuildHasher,
{
    fn eq(&self, other: &Self) -> bool {
        (self.data == other.data) & (self.keys == other.keys)
    }
}

impl<K, V> HashKeylist<K, V, RandomState>
where
    K: Hash + Eq,
    V: Eq,
{
    pub fn new() -> Self {
        HashKeylist {
            data: HashMap::new(),
            keys: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        HashKeylist {
            data: HashMap::with_capacity(capacity),
            keys: Vec::with_capacity(capacity),
        }
    }
}

impl<K, V, S> HashKeylist<K, V, S>
where
    K: Hash + Eq,
    V: Eq,
    S: BuildHasher,
{
    pub fn with_hasher(hash_builder: S) -> Self {
        HashKeylist {
            data: HashMap::with_hasher(hash_builder),
            keys: Vec::new(),
        }
    }

    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        HashKeylist {
            data: HashMap::with_capacity_and_hasher(capacity, hash_builder),
            keys: Vec::with_capacity(capacity),
        }
    }

    pub fn iter(&self) -> Iter<K, V> {
        let map = HashMap::from_iter(
            self.data
                .iter()
                .map(|(k, vs)| (k, RowIter { values: vs.iter() })),
        );
        Iter {
            keys: self.keys.iter(),
            map: map,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        let map = HashMap::from_iter(self.data.iter_mut().map(|(k, vs)| {
            (
                k,
                RowIterMut {
                    values: vs.iter_mut(),
                },
            )
        }));
        IterMut {
            keys: self.keys.iter(),
            map: map,
        }
    }

    pub fn keys<'a>(&'a self) -> impl Iterator<Item = &'a K> {
        self.keys.iter()
    }

    pub fn values<'a>(&'a self) -> impl Iterator<Item = &'a V> {
        self.iter().map(|(_, v)| v)
    }

    pub fn values_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut V> {
        self.iter_mut().map(|(_, v)| v)
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn pop(&mut self) -> Option<(K, V)> {
        let key = self.keys.pop()?;
        let value = self.pop_and_clean(&key)?;
        Some((key, value))
    }

    pub fn remove(&mut self, index: usize) -> (K, V) {
        let key = self.keys.remove(index);
        let value = self.remove_and_clean(&key, index).unwrap();
        (key, value)
    }

    fn pop_and_clean(&mut self, key: &K) -> Option<V> {
        let list = self.get_all_mut(key)?;
        let value = list.pop();
        if list.is_empty() {
            self.data.remove(key);
        }
        value
    }

    fn remove_and_clean(&mut self, key: &K, index: usize) -> Option<V> {
        let pos = self.index_to_position(key, index);
        let list = self.get_all_mut(key)?;
        let value = list.remove(pos);
        if list.is_empty() {
            self.data.remove(key);
        }
        Some(value)
    }

    fn index_to_position(&self, key: &K, index: usize) -> usize {
        self.iter().take(index).filter(|(k, _)| k == &key).count()
    }

    pub fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        let (k, vs) = self.data.get_key_value(key)?;
        Some((k, vs.first()?))
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)?.first()
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.data.get_mut(key)?.first_mut()
    }

    pub fn get_all(&self, key: &K) -> Option<&Vec<V>> {
        self.data.get(key)
    }

    /// You probably only want to use this if you want to change the values in the list. Because if you push to the mutable list it won't get added to the keys list, so for that case just use the `push` function.
    pub fn get_all_mut(&mut self, key: &K) -> Option<&mut Vec<V>> {
        self.data.get_mut(key)
    }

    pub fn get_all_key_value<'a>(&'a self, key: &'a K) -> IterKeyValue<'a, K, V> {
        match self.data.get_key_value(key) {
            Some((x, y)) => IterKeyValue {
                key: x,
                values: y.iter(),
            },
            None => IterKeyValue {
                key,
                values: [].iter(),
            },
        }
    }
}

impl<K, V, S> HashKeylist<K, V, S>
where
    K: Hash + Eq + Clone,
    V: Eq,
    S: BuildHasher,
{
    pub fn insert(&mut self, index: usize, key: K, value: V) {
        let pos = self.index_to_position(&key, index);
        let entry = self.data.entry(key.clone()).or_insert(Vec::new());
        entry.insert(pos, value);
        self.keys.insert(index, key);
    }

    pub fn push(&mut self, k: K, v: V) {
        let entry = self.data.entry(k.clone()).or_insert(Vec::new());
        entry.push(v);
        self.keys.push(k)
    }
}

impl<K, V, S> HashKeylist<K, V, S>
where
    K: Hash + Eq + Clone + std::cmp::Ord,
    V: Eq,
    S: BuildHasher,
{
    pub fn sort_by_key<'a>(&'a mut self) {
        self.keys.sort_unstable()
    }
}

impl<K, V, S> HashKeylist<K, V, S>
where
    K: Hash + Eq + Clone + std::cmp::Ord,
    V: Eq + std::cmp::Ord,
    S: BuildHasher,
{
    pub fn sort<'a>(&'a mut self) {
        self.sort_by_key();
        for item in self.data.values_mut() {
            item.sort();
        }
    }
}

impl<K, V> From<Vec<(K, V)>> for HashKeylist<K, V, RandomState>
where
    K: Hash + Eq + Clone,
    V: Eq,
{
    fn from(input: Vec<(K, V)>) -> Self {
        let mut keys = Vec::with_capacity(input.len());
        let mut map = HashMap::new();
        for (k, v) in input {
            let entry = map.entry(k.clone()).or_insert(Vec::new());
            entry.push(v);
            keys.push(k)
        }

        HashKeylist { data: map, keys }
    }
}

impl<K, V> From<HashKeylist<K, V, RandomState>> for Vec<(K, V)>
where
    K: Hash + Eq + Clone,
    V: Eq,
{
    fn from(input: HashKeylist<K, V, RandomState>) -> Vec<(K, V)> {
        input.into_iter().collect()
    }
}

impl<K, V> std::iter::FromIterator<(K, V)> for HashKeylist<K, V, RandomState>
where
    K: Hash + Eq + Clone,
    V: Eq,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let (size, _) = iter.size_hint();
        let mut keys = Vec::with_capacity(size);
        let mut map = HashMap::new();
        for (k, v) in iter {
            let entry = map.entry(k.clone()).or_insert(Vec::new());
            entry.push(v);
            keys.push(k)
        }

        HashKeylist { data: map, keys }
    }
}

impl<K, V, S> Extend<(K, V)> for HashKeylist<K, V, S>
where
    K: Eq + Hash + Clone,
    V: Eq,
    S: BuildHasher,
{
    #[inline]
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (k, v) in iter {
            let entry = self.data.entry(k.clone()).or_insert(Vec::new());
            entry.push(v);
            self.keys.push(k);
        }
    }
}

impl<K, V, S> Clone for HashKeylist<K, V, S>
where
    K: Hash + Eq + Clone,
    V: Eq + Clone,
    S: BuildHasher + Clone,
{
    fn clone(&self) -> Self {
        HashKeylist {
            data: self.data.clone(),
            keys: self.keys.clone(),
        }
    }
}

#[cfg(feature = "serde")]
mod serde {
    use crate::HashKeylist;
    use serde::de::{Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
    use serde::ser::{Serialize, Serializer};
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hash};
    use std::marker::PhantomData;

    impl<K, V, H> Serialize for HashKeylist<K, V, H>
    where
        K: Serialize + Hash + Eq,
        V: Serialize + Eq,
        H: BuildHasher,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_newtype_struct("HashKeylist", &self.iter().collect::<Vec<_>>())
        }
    }

    struct KeylistVisitor<K, V, H>
    where
        K: Hash + Eq,
        V: Eq,
        H: BuildHasher,
    {
        marker: PhantomData<fn() -> HashKeylist<K, V, H>>,
    }

    impl<K, V, H> KeylistVisitor<K, V, H>
    where
        K: Hash + Eq,
        V: Eq,
        H: BuildHasher,
    {
        fn new() -> Self {
            KeylistVisitor {
                marker: PhantomData,
            }
        }
    }

    impl<'de, K, V> Visitor<'de> for KeylistVisitor<K, V, RandomState>
    where
        K: Deserialize<'de> + Hash + Eq + Clone,
        V: Deserialize<'de> + Eq,
    {
        type Value = HashKeylist<K, V, RandomState>;
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

            Ok(HashKeylist::from(buffer))
        }

        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut buffer = Vec::new();

            while let Some(x) = access.next_entry()? {
                buffer.push(x)
            }
            Ok(HashKeylist::from(buffer))
        }

        fn visit_newtype_struct<D>(
            self,
            deserializer: D,
        ) -> Result<HashKeylist<K, V, RandomState>, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_newtype_struct("HashKeylist", KeylistVisitor::new())
        }
    }

    impl<'de, K, V> Deserialize<'de> for HashKeylist<K, V, RandomState>
    where
        K: Deserialize<'de> + Hash + Eq + Clone,
        V: Deserialize<'de> + Eq,
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
    use crate::HashKeylist;
    use std::collections::hash_map::RandomState;
    use std::collections::HashMap;
    use std::iter::FromIterator;

    fn data() -> HashKeylist<&'static str, u32, RandomState> {
        HashKeylist {
            data: HashMap::from_iter(vec![("oke", vec![1, 2]), ("test", vec![19])]),
            keys: vec!["oke", "test", "oke"],
        }
    }

    #[test]
    fn from() {
        let keylist = HashKeylist::from(vec![("oke", 1), ("test", 19), ("oke", 2)]);
        let expected = data();

        assert_eq!(keylist, expected);
    }

    #[test]
    fn iter() {
        let keylist = data();

        let mut iter = keylist.iter();
        assert_eq!(Some((&"oke", &1)), iter.next());
        assert_eq!(Some((&"test", &19)), iter.next());
        assert_eq!(Some((&"oke", &2)), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn into_iter() {
        let keylist = data();

        let mut iter = keylist.into_iter();
        assert_eq!(Some(("oke", 1)), iter.next());
        assert_eq!(Some(("test", 19)), iter.next());
        assert_eq!(Some(("oke", 2)), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn extend() {
        let mut keylist = data();

        keylist.extend(vec![("oke", 3), ("testing", 918), ("test", 55)]);

        let expected = HashKeylist {
            data: HashMap::from_iter(vec![
                ("oke", vec![1, 2, 3]),
                ("test", vec![19, 55]),
                ("testing", vec![918]),
            ]),
            keys: vec!["oke", "test", "oke", "oke", "testing", "test"],
        };

        assert_eq!(keylist, expected);
    }

    #[test]
    fn get_all_key_value() {
        let keylist = data();

        let list: Vec<_> = keylist.get_all_key_value(&"oke").collect();
        let expected = vec![(&"oke", &1), (&"oke", &2)];

        assert_eq!(list, expected);
    }

    #[test]
    fn push() {
        let mut keylist = data();

        keylist.push("oke", 3);

        let expected = HashKeylist {
            data: HashMap::from_iter(vec![("oke", vec![1, 2, 3]), ("test", vec![19])]),
            keys: vec!["oke", "test", "oke", "oke"],
        };

        assert_eq!(keylist, expected);

        keylist.push("testing", 120);

        let expected = HashKeylist {
            data: HashMap::from_iter(vec![
                ("oke", vec![1, 2, 3]),
                ("test", vec![19]),
                ("testing", vec![120]),
            ]),
            keys: vec!["oke", "test", "oke", "oke", "testing"],
        };

        assert_eq!(keylist, expected);
    }

    #[test]
    fn insert() {
        let mut keylist = data();

        keylist.insert(1, "oke", 3);

        let expected = HashKeylist {
            data: HashMap::from_iter(vec![("oke", vec![1, 3, 2]), ("test", vec![19])]),
            keys: vec!["oke", "oke", "test", "oke"],
        };

        assert_eq!(keylist, expected);
    }

    #[test]
    fn insert_2() {
        let mut keylist = HashKeylist::<_, _, RandomState> {
            data: HashMap::from_iter(vec![
                ("oke", vec![1, 2, 3, 4, 5]),
                ("test", vec![19, 21, 23]),
            ]),
            keys: vec!["oke", "oke", "test", "oke", "test", "oke", "oke", "test"],
        };

        keylist.insert(3, "oke", 1234);

        let expected = HashKeylist {
            data: HashMap::from_iter(vec![
                ("oke", vec![1, 2, 1234, 3, 4, 5]),
                ("test", vec![19, 21, 23]),
            ]),
            keys: vec![
                "oke", "oke", "test", "oke", "oke", "test", "oke", "oke", "test",
            ],
        };
        assert_eq!(keylist, expected);

        let expected = HashKeylist {
            data: HashMap::from_iter(vec![
                ("oke", vec![1, 2, 1234, 3, 4, 5]),
                ("test", vec![19, 21, 23]),
                ("testing", vec![901]),
            ]),
            keys: vec![
                "oke", "oke", "test", "testing", "oke", "oke", "test", "oke", "oke", "test",
            ],
        };

        keylist.insert(3, "testing", 901);

        assert_eq!(keylist, expected);
    }

    #[test]
    fn pop() {
        let mut keylist = data();

        assert_eq!(Some(("oke", 2)), keylist.pop());
        assert_eq!(Some(("test", 19)), keylist.pop());
        assert_eq!(Some(("oke", 1)), keylist.pop());
        assert_eq!(None, keylist.pop());
    }

    #[test]
    fn remove_index() {
        let mut keylist = data();

        assert_eq!(("test", 19), keylist.remove(1));
        assert_eq!(("oke", 2), keylist.remove(1));
        assert_eq!(("oke", 1), keylist.remove(0));
        assert_eq!(None, keylist.pop());
    }

    #[test]
    fn remove_first() {
        let mut keylist = data();

        assert_eq!(("oke", 1), keylist.remove(0));
        assert_eq!(("test", 19), keylist.remove(0));
        assert_eq!(("oke", 2), keylist.remove(0));
        assert_eq!(None, keylist.pop());
    }

    #[test]
    #[should_panic(expected = "removal index (is 0) should be < len (is 0)")]
    fn remove_empty() {
        let mut keylist: HashKeylist<u8, u8, RandomState> = HashKeylist::new();
        keylist.remove(0);
    }

    #[test]
    fn is_empty() {
        let keylist: HashKeylist<u8, u8, RandomState> = HashKeylist::new();
        assert!(keylist.is_empty());

        let keylist = data();
        assert!(!keylist.is_empty())
    }

    #[test]
    fn len() {
        let keylist: HashKeylist<u8, u8, RandomState> = HashKeylist::new();
        assert_eq!(0, keylist.len());

        let keylist = data();
        assert_eq!(3, keylist.len());
    }

    #[test]
    fn get() {
        let keylist = data();
        assert_eq!(Some(&1), keylist.get(&"oke"));
    }

    #[test]
    fn get_all() {
        let keylist = data();
        assert_eq!(Some(&vec![1, 2]), keylist.get_all(&"oke"));
    }

    #[test]
    fn get_mut() {
        let mut keylist = data();
        let expected = HashKeylist {
            data: HashMap::from_iter(vec![("oke", vec![14, 2]), ("test", vec![38])]),
            keys: vec!["oke", "test", "oke"],
        };

        let item = keylist.get_mut(&"oke").unwrap();
        *item += 13;

        let item = keylist.get_mut(&"test").unwrap();
        *item *= 2;

        assert_eq!(expected, keylist);
    }

    #[test]
    fn keys() {
        let keylist = data();
        assert_eq!(
            vec![&"oke", &"test", &"oke"],
            keylist.keys().collect::<Vec<_>>()
        );
    }

    #[test]
    fn values() {
        let keylist = data();
        assert_eq!(vec![&1, &19, &2], keylist.values().collect::<Vec<_>>());
    }

    #[test]
    fn values_mut() {
        let mut keylist = data();
        let expected = HashKeylist {
            data: HashMap::from_iter(vec![("oke", vec![2, 4]), ("test", vec![38])]),
            keys: vec!["oke", "test", "oke"],
        };

        for val in keylist.values_mut() {
            *val *= 2;
        }

        assert_eq!(expected, keylist);
    }

    #[test]
    fn sort_by_key() {
        let mut keylist: HashKeylist<_, _, RandomState> = HashKeylist {
            data: HashMap::from_iter(vec![("oke", vec![2, 1]), ("test", vec![19])]),
            keys: vec!["oke", "test", "oke"],
        };
        let expected = HashKeylist {
            data: HashMap::from_iter(vec![("oke", vec![2, 1]), ("test", vec![19])]),
            keys: vec!["oke", "oke", "test"],
        };
        keylist.sort_by_key();
        assert_eq!(expected, keylist);
    }

    #[test]
    fn sort() {
        let mut keylist: HashKeylist<_, _, RandomState> = HashKeylist {
            data: HashMap::from_iter(vec![("oke", vec![2, 3, 1]), ("test", vec![21, 19])]),
            keys: vec!["oke", "test", "oke", "test", "oke"],
        };
        let expected = HashKeylist {
            data: HashMap::from_iter(vec![("oke", vec![1, 2, 3]), ("test", vec![19, 21])]),
            keys: vec!["oke", "oke", "oke", "test", "test"],
        };
        keylist.sort();
        assert_eq!(expected, keylist);
    }

    #[test]
    fn multi() {
        use std::collections::HashMap;
        use std::iter::FromIterator;

        let mut map = HashMap::new();
        map.insert("one", 1);
        map.insert("two", 2);
        map.insert("three", 3);
        map.insert("four", 4);

        let mut keylist = HashKeylist::from_iter(map);
        // sorts keys alphabetically
        keylist.sort_by_key();

        keylist.push("one", 11);
        keylist.push("five", 5);
        keylist.push("five", 1);

        assert_eq!(
            vec![
                (&"four", &4),
                (&"one", &1),
                (&"three", &3),
                (&"two", &2),
                (&"one", &11),
                (&"five", &5),
                (&"five", &1),
            ],
            keylist.iter().collect::<Vec<_>>()
        );

        keylist.insert(2, "five", 12);

        assert_eq!(
            vec![
                (&"four", &4),
                (&"one", &1),
                (&"five", &12),
                (&"three", &3),
                (&"two", &2),
                (&"one", &11),
                (&"five", &5),
                (&"five", &1),
            ],
            keylist.iter().collect::<Vec<_>>()
        );

        assert_eq!(Some(("five", 1)), keylist.pop());

        assert_eq!(
            vec![
                (&"four", &4),
                (&"one", &1),
                (&"five", &12),
                (&"three", &3),
                (&"two", &2),
                (&"one", &11),
                (&"five", &5),
            ],
            keylist.iter().collect::<Vec<_>>()
        );

        assert_eq!(("two", 2), keylist.remove(4));

        assert_eq!(
            vec![
                (&"four", &4),
                (&"one", &1),
                (&"five", &12),
                (&"three", &3),
                (&"one", &11),
                (&"five", &5),
            ],
            keylist.iter().collect::<Vec<_>>()
        );

        assert_eq!(
            vec![&"four", &"one", &"five", &"three", &"one", &"five"],
            keylist.keys().collect::<Vec<_>>()
        );

        assert_eq!(
            vec![&4, &1, &12, &3, &11, &5],
            keylist.values().collect::<Vec<_>>()
        );

        assert_eq!(
            vec![
                ("four", 4),
                ("one", 1),
                ("five", 12),
                ("three", 3),
                ("one", 11),
                ("five", 5),
            ],
            Vec::from(keylist)
        )
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use crate::HashKeylist;
    use serde_test::{assert_de_tokens, assert_ser_tokens, assert_tokens, Token};

    #[test]
    fn serde_de_list() {
        let expected = HashKeylist::from(vec![("oke", 1), ("test", 15)]);

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
        let expected = HashKeylist::from(vec![("oke", 1), ("test", 15)]);

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
        let input = HashKeylist::from(vec![("oke", 1), ("test", 15)]);

        assert_ser_tokens(
            &input,
            &[
                Token::NewtypeStruct {
                    name: "HashKeylist",
                },
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
        let input = HashKeylist::from(vec![("oke", 1), ("test", 15)]);

        assert_tokens(
            &input,
            &[
                Token::NewtypeStruct {
                    name: "HashKeylist",
                },
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
