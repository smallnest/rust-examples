// Idiom #60 Read command line argument
// Assign to x the string value of the first command line parameter, after the program name.

use std::env;

fn main() {
    let x = env::args().nth(1).unwrap_or("".to_string());
    println!("Argument was: {}", x);

    let first_arg = env::args().skip(1).next();

    let fallback = "".to_owned();
    let x = first_arg.unwrap_or(fallback);
    
    println!("{:?}", x);
}