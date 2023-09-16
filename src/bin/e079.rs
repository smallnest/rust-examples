// Idiom #79 Convert integer to floating point number
// Declare the floating point number y and initialize it with the value of the integer x .

fn main() {
    let x = 5;
    let y = x as f64;

    println!("int {:?}, float {:?}", x, y);
}
