// Idiom #59 Write to standard error stream
// Print the message "x is negative" to standard error (stderr), with integer x value substitution (e.g. "-2 is negative").

fn main() {
    let x = -3;
    eprintln!("{} is negative", x);
}
