// Idiom #8 Create a map (associative array)
// Create a new map object x, and provide some (key, value) pairs as initial content.
use std::collections::HashMap;
use std::collections::BTreeMap;
use map_macro::hash_map;


fn main() {
    let x: HashMap<&str, i32> = [
        ("one", 1),
        ("two", 2),
    ].into_iter().collect();
    
    println!("{:?}", x);

    let mut x = BTreeMap::new();
    x.insert("one", 1);
    x.insert("two", 2);
    
    println!("{:?}", x);

    let m = HashMap::from([("one", 1), ("two", 2)]);
    println!("{:?}", m);

    let m = BTreeMap::from([("one", 1), ("two", 2)]);
    println!("{:?}", m);

    let m = hash_map! {
        "one" => 1,
        "two" => 2,
    };
    println!("{:?}", m);
}