// Idiom #93 Pass a runnable procedure as parameter
// Implement the procedure control which receives one parameter f, and runs f.

fn control(f: impl Fn()) {
    f();
}

fn hello() {
    println!("Hello,");
}

fn main() {
    control(hello);
    control(|| { println!("Is there anybody in there?"); });
}
