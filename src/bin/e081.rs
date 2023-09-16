// Idiom #81 Round floating point number to integer
// Declare the integer y and initialize it with the rounded value of the floating point number x .
// Ties (when the fractional part of x is exactly .5) must be rounded up (to positive infinity).


fn main() {
    let x : f64 = 2.71828;
    let y = x.round() as i64;
    
    println!("{} {}", x, y);
}