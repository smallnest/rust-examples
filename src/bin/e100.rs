// Idiom #100 Sort by a comparator
// Sort elements of array-like collection items, using a comparator c.

fn main() {
    let mut items = [1, 7, 5, 2, 3];
    items.sort_by(i32::cmp);
    println!("{:?}", items);
}