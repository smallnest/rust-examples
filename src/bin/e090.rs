// Idiom #90 Read-only outside
// Expose a read-only integer x to the outside world while being writable inside a structure or a class Foo.


struct Foo {
    x: usize
}

impl Foo {
    pub fn new(x: usize) -> Self {
        Foo { x }
    }

    pub fn x(& self) -> & usize {
        &self.x
    }
}

fn main() {
    let foo = Foo::new(42);
    println!("foo.x = {}", foo.x());
}