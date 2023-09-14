// Idiom #67 Binomial coefficient "n choose k"
// Calculate binom(n, k) = n! / (k! * (n-k)!). Use an integer type able to handle huge numbers.


use num::bigint::BigInt;
use num::bigint::ToBigInt;
use num::traits::One;

fn binom(n: u64, k: u64) -> BigInt {
    let mut res = BigInt::one();
    for i in 0..k {
        res = (res * (n - i).to_bigint().unwrap()) /
              (i + 1).to_bigint().unwrap();
    }
    res
}

fn main() {
    let n = 133;
    let k = 71;

    println!("{}", binom(n, k));
}
