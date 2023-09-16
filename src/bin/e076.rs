// Idiom #76 Binary digits from an integer
// Create the string s of integer x written in base 2.
//
// E.g. 13 -> "1101"

fn main() {
    let x = 13;
    let s = format!("{x:b}");

    println!("{}", s);

    let s = format!("{:b}", x);
    println!("{}", s)
}
