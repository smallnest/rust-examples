// Idiom #49 Split a space-separated string
// Build list chunks consisting in substrings of the string s, separated by one or more space characters.

fn main() {
    let s = "What a  mess";

    let chunks: Vec<_> = s.split(' ').collect();

    println!("{:?}", chunks);
}
