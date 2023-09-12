// Idiom #38 Extract a substring
// Find substring t consisting in characters i (included) to j (excluded) of string s.
// Character indices start at 0 unless specified otherwise.
// Make sure that multibyte characters are properly handled.

use substring::Substring;

fn main() {
    let s = "hello 世界";
    let t = s.substring(6, 8);
    println!("{}", t);
}