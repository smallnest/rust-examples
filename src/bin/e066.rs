// Idiom #66 Big integer exponentiation
// Calculate the result z of x power n, where x is a big integer and n is a positive integer.

extern crate num;

use num::bigint::BigInt;

fn main() {
    let x = BigInt::parse_bytes(b"600000000000", 10).unwrap();
    let n = 42;
    let z = num::pow(x, n);
    println!("{}", z)
}