use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");

    println!("{:?}", map.get("key1"));
    println!("{:?}", map.get("key2"));

    let x = 1;
}
