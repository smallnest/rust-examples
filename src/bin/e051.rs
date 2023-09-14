// Idiom #51 Check if map contains key
// Determine whether the map m contains an entry for the key k

use std::collections::HashMap;

fn main() {
    let mut m = HashMap::new();
    m.insert(1, "a");
    m.insert(2, "b");

    let k = 2;

    let hit = m.contains_key(&k);

    println!("{:?}", hit);
}
