// Idiom #92 Save object into JSON file
//Write the contents of the object x into the file data.json.


use std::fs::File;
use serde;
use serde_json;
use serde::{Serialize,Deserialize};


#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
    city: String,
}

fn main() {
    let u = User {
        name: "John".to_string(),
        age: 42,
        city: "London".to_string(),
    };
    
    let f = File::create("data.json").unwrap();

    match serde_json::to_writer(&f, &u) {
        Ok(_) => println!("Success!"),
        Err(e) => println!("Error! {}", e),
    }
}