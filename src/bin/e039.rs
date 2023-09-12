// Idiom #39 Check if string contains a word
// Set the boolean ok to true if the string word is contained in string s as a substring, or to false otherwise.


fn main() {
    let s = "Let's dance the macarena";

    let word = "dance";
    let ok = s.contains(word);
    println!("{}", ok);

    let word = "car";
    let ok = s.contains(word);
    println!("{}", ok);

    let word = "duck";
    let ok = s.contains(word);
    println!("{}", ok);
}
