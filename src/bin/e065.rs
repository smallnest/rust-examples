// Idiom #65 Format decimal number
// From the real value x in [0,1], create its percentage string representation s with one digit after decimal point. E.g. 0.15625 -> "15.

fn main() {
    let x = 0.15625f64;
    let s = format!("{:.1}%", 100.0 * x);
    
    println!("{}", s);
}
