// Idiom #97 Check string suffix
// Set boolean b to true if string s ends with string suffix, false otherwise.

fn main() {
    let s = "bananas";
    let suffix = "nas";

    let b = s.ends_with(suffix);

    println!("{:?}", b);
}
