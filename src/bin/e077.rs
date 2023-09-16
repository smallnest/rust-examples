// Idiom #77 Complex number
// Declare a complex x and initialize it with value (3i - 2). Then multiply it by i.

use num::Complex;

fn main() {
    let mut x = Complex::new(-2, 3);
    x *= Complex::i();
    println!("{}", x);
}

