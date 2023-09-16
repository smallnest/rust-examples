// Idiom #75 Compute LCM
// Compute the least common multiple x of big integers a and b. Use an integer type able to handle huge numbers.

use num::bigint::BigInt;
use num::Integer;

fn main() {
    let a = BigInt::parse_bytes(b"6000000000000", 10).unwrap();
    let b = BigInt::parse_bytes(b"9000000000000", 10).unwrap();
    let x = a.lcm(&b);
    println!("x = {}", x);
}
