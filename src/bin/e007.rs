// Idiom #7 Iterate over list indexes and values
// Print each index i with its value x from an array-like collection items
fn main() {
    let items = ["a", "b", "c"];
    items.iter().enumerate().for_each(|(i, x)| {
        println!("Item {} = {}", i, x);
    });

    for (i, x) in items.iter().enumerate() {
        println!("Item {} = {}", i, x);
    }
}
