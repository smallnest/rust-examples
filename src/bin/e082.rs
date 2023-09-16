// Idiom #82 Count substring occurrences
// Find how many times string s contains substring t.
// Specify if overlapping occurrences are counted.

fn main() {
    let s = "lorem ipsum lorem ipsum lorem ipsum lorem ipsum";
    let t = "ipsum";

    let c = s.matches(t).count();

    println!("{} occurrences", c);
}

