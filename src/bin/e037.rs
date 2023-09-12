// Idiom #37 Currying
// Transform a function that takes multiple arguments into a function for which some of the arguments are preset.

fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn main() {
    let add5 = move |x| add(5, x);

    let y = add5(12);
    println!("{}", y);
}
