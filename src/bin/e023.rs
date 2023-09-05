// Idiom #23 Convert real number to string with 2 decimal places
// Given a real number x, create its string representation s with 2 decimal digits following the dot.

fn main() {
    let x = 42.1337;
    let s = format!("{:.2}", x);
    
    println!("{}", s);
}
