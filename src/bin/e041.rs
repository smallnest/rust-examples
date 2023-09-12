// Idiom #41 Reverse a string
// Create string t containing the same characters as string s, in reverse order.
// Original string s must remain unaltered. Each character must be handled correctly regardless its number of bytes in memory.

fn main() {
    let s = "hello 世界";
    let t: String = s.chars().rev().collect();
    println!("{}", t);
}
