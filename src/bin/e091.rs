// Idiom #91 Load JSON file into object
// Read from the file data.json and write its content into the object x.
// Assume the JSON data is suitable for the type of x.

use serde;
use serde_json;
use serde::Deserialize;

use std::error::Error;
use std::fs::File;
use std::path::Path;
#[derive(Debug, Deserialize)]
struct User {
    name: String,
    age: u32,
    city: String,
}

fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<User, Box<dyn Error>> {
    // Open the file in read-only mode.
    let file = File::open(path)?;

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(file)?;

    // Return the `User`.
    Ok(u)
}

fn main() {
    let u = read_user_from_file("test.json").unwrap();
    println!("{:#?}", u);
}