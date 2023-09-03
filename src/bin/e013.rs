// Idiom #13 Iterate over map keys and values
// Access each key k with its value x from an associative array mymap, and print them.

use std::collections::BTreeMap;

fn main() {
    let mut mymap = BTreeMap::new();
    mymap.insert("one", 1);
    mymap.insert("two", 2);
    mymap.insert("three", 3);
    mymap.insert("four", 4);

    for (k, x) in &mymap {
        println!("Key={key}, Value={val}", key = k, val = x);
    }
}
