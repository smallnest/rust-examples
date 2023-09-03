// Idiom #19 Reverse a list
// Reverse the order of the elements of the list x.
// This may reverse "in-place" and destroy the original ordering.

fn main() {
    let mut x = vec!["Hello", "World"];
    let y: Vec<_> = x.iter().rev().collect();
    println!("y = {:?}", y);

    // x is left unchanged
    println!("x = {:?}", x);

    x.reverse();
    println!("x = {:?}", x);
}
