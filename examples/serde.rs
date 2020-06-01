extern crate serde_json;

use keylist::{HashKeylist, Keylist};

fn main() {
    let map_text = r#"
    {
        "test": 1,
        "another": 123,
        "key": 102
    }
    "#;

    let vec_text = r#"
    [
        ["test", 1],
        ["another", 123],
        ["another", 125],
        ["test", 6],
        ["key", 5],
        ["test", 2],
        ["key", 2]
    ]
    "#;
    let value: Keylist<&str, u32> = serde_json::from_str(map_text).unwrap();
    println!("{:?}", value);
    let value: Keylist<&str, u32> = serde_json::from_str(vec_text).unwrap();
    println!("{:?}", value);

    println!("{}", serde_json::to_string(&value).unwrap());

    let value: HashKeylist<&str, u32, _> = serde_json::from_str(map_text).unwrap();
    println!("{:?}", value);
    let value: HashKeylist<&str, u32, _> = serde_json::from_str(vec_text).unwrap();
    println!("{:?}", value);

    println!("{}", serde_json::to_string(&value).unwrap());
}
