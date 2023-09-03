// Idiom #6 Iterate over list values
// Do something with each item x of the list (or array) items, regardless indexes.
fn main() {
    let items = vec![11, 22, 33];

    items.clone().into_iter().for_each(|x| do_something(x));

    for x in items {
        do_something(x);
    }
}

fn do_something(n: i64) {
    println!("Number {}", n)
}
