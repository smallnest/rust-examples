// Idiom #80 Truncate floating point number to integer
// Declare integer y and initialize it with the value of floating point number x . Ignore non-integer digits of x .
// Make sure to truncate towards zero: a negative x must yield the closest greater integer (not lesser).


fn main() {
    let x = 41.59999999f64;
    let y = x as i32;
    println!("{}", y);
}