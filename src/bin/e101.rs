// Idiom #101 Load from HTTP GET request into a string
// Make an HTTP request with method GET to the URL u, then store the body of the response in the string s.

use reqwest;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get("http://httpbin.org/get")?
    .text()?;

    println!("get body = {:?}", body);


    let mut map = HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    let client = reqwest::blocking::Client::new();
    let res = client.post("http://httpbin.org/post")
        .json(&map)
        .send();

    println!("post Response: {:?}", res?);

    Ok(())
}