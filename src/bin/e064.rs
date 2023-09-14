// Idiom #64 Big integer : value 3 power 247
// Assign to x the value 3^247

use num::bigint::ToBigInt;

fn main() {
    let a = 3.to_bigint().unwrap();
    let x = num::pow(a, 247);
    println!("{}", x)
}
