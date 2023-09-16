// Idiom #74 Compute GCD
// Compute the greatest common divisor x of big integers a and b. Use an integer type able to handle huge numbers.

use num::Integer;
use num::bigint::BigInt;

fn main() {
    let a = BigInt::parse_bytes(b"6000000000000", 10).unwrap();
    let b = BigInt::parse_bytes(b"9000000000000", 10).unwrap();
    
    let x = a.gcd(&b);
 
    println!("{}", x);
}
