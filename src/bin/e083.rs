// Idiom #83 Regex with character repetition
// Declare regular expression r matching strings "http", "htttp", "httttp", etc.

use regex::Regex;

fn main() {
    let r = Regex::new(r"htt+p").unwrap();
    
    assert!(r.is_match("http"));
    assert!(r.is_match("htttp"));
    assert!(r.is_match("httttp"));
}