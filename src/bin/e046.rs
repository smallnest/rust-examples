// Idiom #46 Extract beginning of string (prefix)
// Create string t consisting of the 5 first characters of string s.
// Make sure that multibyte characters are properly handled.

fn main() {
    let s = "hello 世界";
    
    let t = s.chars().take(5).collect::<String>();

    println!("{}", t);
}
