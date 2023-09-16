// Idiom #96 Check string prefix
// Set boolean b to true if string s starts with prefix prefix, false otherwise.

fn main() {
    let s = "bananas";
    let prefix = "bana";

    let b = s.starts_with(prefix);

    println!("{:?}", b);
}
