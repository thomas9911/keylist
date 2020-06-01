# keylist

### Keylist
Map like wrapper around a list with tuple pairs. Inspired by Elixir's Keyword lists.

Because it is just a list with tuples, keys can be anything that can be put in a tuple. Also means that getting a value is not efficient.

For something more usefull and quite similar take a look at: https://docs.rs/multimap
```rust
use keylist::Keylist;

let mut keylist = Keylist::from(vec![("a", 5), ("b", 2), ("a", 1)]);

assert_eq!(keylist.get(&"a"), Some(&5));

keylist.sort_by_value();

assert_eq!(keylist.get(&"a"), Some(&1));

keylist.push("z", 26);

assert_eq!(keylist.get(&"z"), Some(&26));

keylist.insert(1, "z", 2);

assert_eq!(keylist.get(&"z"), Some(&2));

assert_eq!(keylist.get_all(&"z"), vec![&2, &26]);

assert_eq!(keylist.get_key_value(&"b"), Some(&("b", 2)));

let mut swapped_keylist = keylist.into_swapped();

assert_eq!(swapped_keylist.get(&2), Some(&"z"));

assert_eq!(swapped_keylist.get_all(&2), vec![&"z", &"b"]);

swapped_keylist.sort();

assert_eq!(swapped_keylist.get_all(&2), vec![&"b", &"z"]);

swapped_keylist.extend(vec![(3, "b"), (2, "g")]);

assert_eq!(swapped_keylist.get_all(&2), vec![&"b", &"z", &"g"]);

```

Convert keylist to map and back:
```rust
use std::collections::HashMap;
use std::iter::FromIterator;
use keylist::Keylist;

let keylist = Keylist::from(vec![("a", 5), ("b", 2), ("c", 1)]);

let mut map = HashMap::new();
map.extend(vec![("a", 5), ("b", 2), ("c", 1)]);

let map_from_keylist = HashMap::from_iter(keylist.clone());
let mut keylist_from_map = Keylist::from_iter(map.clone());
keylist_from_map.sort();

assert_eq!(map, map_from_keylist);
assert_eq!(keylist, keylist_from_map);
```

Arbitrary example:
```rust
use keylist::Keylist;

let mut keylist = Keylist::new();
keylist.push(vec![3.12, 0.12], "a");
keylist.push(vec![0.1235, 34.121551], "c");

assert_eq!(keylist.get(&vec![0.1235, 34.121551]), Some(&"c"));


```

### HashKeylist
There is also a more efficient implemetation, backend by a hashmap, therefore the keys should be hashable.
```rust
use keylist::HashKeylist;
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
```

License: Unlicense
