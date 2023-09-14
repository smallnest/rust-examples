// Idiom #58 Extract file content to a string
// Create string lines from the content of the file with filename f.

use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), ()> {
    let f = "Cargo.toml";

    let mut file = File::open(f).expect("Can't open file.");
    let mut lines = String::new();
    file.read_to_string(&mut lines)
        .expect("Can't read file contents.");

    println!("{}", lines);

    Ok(())
}
