
// Idiom #110 Check if string is blank
// Set the boolean blank to true if the string s is empty, or null, or contains only whitespace ; false otherwise.

fn main() {
    let list = vec!["", " ", "  ", "\t", "\n", "a", " b "];
    for s in list {
        let blank = s.trim().is_empty();

        if blank {
            println!("{:?}\tis blank", s)
        } else {
            println!("{:?}\tis not blank", s)
        }
    }
}
