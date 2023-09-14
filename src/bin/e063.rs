// Idiom #63 Replace fragment of a string
// Assign to x2 the value of string x with all occurrences of y replaced by z.
// Assume occurrences of y are not overlapping.

fn main() {
    let x = "lorem ipsum dolor lorem ipsum";
    let y = "lorem";
    let z = "LOREM";

    let x2 = x.replace(&y, &z);
    
    println!("{}", x2);
}
