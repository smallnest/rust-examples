// Idiom #52 Check if map contains value
// Determine whether the map m contains an entry with the value v, for some key.

use std::collections::BTreeMap;

fn main() {
    let mut m = BTreeMap::new();
    m.insert(11, "one");
    m.insert(22, "twenty-two");

    {
        let v = "eight";
        let does_contain = m.values().any(|&val| *val == *v);
        println!("{:?}", does_contain);
    }

    {
        let v = "twenty-two";
        let does_contain = m.values().any(|&val| *val == *v);
        println!("{:?}", does_contain);
    }
}
