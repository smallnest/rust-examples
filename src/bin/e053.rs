// Idiom #53 Join a list of strings
// Concatenate elements of string list x joined by the separator ", " to create a single string y.


fn main() {
    let x = vec!["Lorem", "ipsum", "dolor", "sit", "amet"];
    let y = x.join(", ");
    println!("{}", y);
}