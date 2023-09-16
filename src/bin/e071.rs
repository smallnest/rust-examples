// Idiom #71 Echo program implementation
// Basic implementation of the Echo program: Print all arguments except the program name, separated by space, followed by newline.
// The idiom demonstrates how to skip the first argument if necessary, concatenate arguments as strings, append newline and print it to stdout.

use std::env;

fn main() {
    println!("{}", env::args().skip(1).collect::<Vec<_>>().join(" "));
}