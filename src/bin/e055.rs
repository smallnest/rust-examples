// Idiom #55 Convert integer to string
// Create the string representation s (in radix 10) of the integer value i.

fn main() {
    let i = 42;
    let s = i.to_string();
    println!("{}", s);

    let s = format!("{}", i);

    println!("{}", s);
}