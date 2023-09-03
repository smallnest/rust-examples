// Idiom #4 Create a function
// Create a function which returns the square of an integer
fn main() {
    let x = 5;
    let y = square(x);
    println!("{} squared is {}", x, y);
}

fn square(x: u32) -> u32 {
    x * x
}
