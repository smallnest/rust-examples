// Idiom #47 Extract string suffix
// Create string t consisting in the 5 last characters of string s.
// Make sure that multibyte characters are properly handled.

fn main() {
    let s = "hello 世界";
    let last5ch = s.chars().count() - 5;
    let s2: String = s.chars().skip(last5ch).collect();
    println!("{}", s2);
}
