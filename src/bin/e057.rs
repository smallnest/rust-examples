 // Idiom #57 Filter list
// Create the list y containing the items from the list x that satisfy the predicate p. Respect the original ordering. Don't modify x in-place.

fn main() {
    let x = vec![1, 2, 3, 4, 5, 6];

    let y: Vec<_> = x.iter()
        .filter(|&x| x % 2 == 0)
        .collect();

    println!("{:?}", y);
}