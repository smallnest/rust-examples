// Idiom #32 Integer exponentiation by squaring
// Create function exp which calculates (fast) the value x power n.
// x and n are non-negative integers.

fn exp(x: u64, n: u64) -> u64 {
    match n {
        0 => 1,
        1 => x,
        i if i % 2 == 0 => exp(x * x, n / 2),
        _ => x * exp(x * x, (n - 1) / 2),
    }
}

fn exp2(x: u64, n: u32) -> u64 {
    x.pow(n)
}

fn main() {
    let x = 16;
    let n = 4;
    
    println!("{}", exp(x, n));
}