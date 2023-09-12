// Idiom #34 Create a set of objects
// Declare and initialize a set x containing unique objects of type T.

use std::collections::HashSet;

fn main() {
    let mut m = HashSet::new();
    m.insert("a");
    m.insert("b");

    println!("{:?}", m);
}
