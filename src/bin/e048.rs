// Idiom #48 Multi-line string literal
// Assign to variable s a string literal consisting in several lines of text, including newlines.


fn main() {
    let s = "line 1
line 2
line 3";
    
    print!("{}", &s);
}