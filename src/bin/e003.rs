// Idiom #3 Create a procedure
// Like a function which doesn't return any value, thus has only side effects (e.g. Print to standard output)
fn main() {
    finish("Jerry");
}

fn finish(name: &str) {
    println!("My job here is done. Goodbye {}", name);
}