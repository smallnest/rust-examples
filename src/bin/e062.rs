// Idiom #62 Find substring position
// Set i to the first position of string y inside string x, if exists.
//
// Specify if i should be regarded as a character index or as a byte index.
//
// Explain the behavior when y is not contained in x.

fn main() {
    let x = "été chaud";
    
    let y = "chaud";
    let i = x.find(y);
    println!("{:?}", i);
    
    let y = "froid";
    let i = x.find(y);
    println!("{:?}", i);
}