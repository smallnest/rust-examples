// Idiom #28 Sort by a property
// Sort the elements of the list (or array-like collection) items in ascending order of x.p, where p is a field of the type Item of the objects in items.


#[derive(Debug)]
struct Foo {
    p: i32,
}

fn main() {
    let mut items = vec![Foo { p: 3 }, Foo { p: 1 }, Foo { p: 2 }, Foo { p: 4 }];
    items.sort_by_key(|x| x.p);
    println!("{:?}", items);

    items.sort_by(|a, b| a.p.cmp(&b.p));
    println!("{:?}", items);
}

