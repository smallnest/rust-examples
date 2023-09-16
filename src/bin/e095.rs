// Idiom #95 Get file size
// Assign to variable x the length (number of bytes) of the local file at path.

use std::fs;

fn filesize(path: &str) -> Result<u64, std::io::Error> {
    let x = fs::metadata(path)?.len();
    Ok(x)
}

fn main() {
    let path = "/etc/hosts";
    let x = filesize(path);
    println!("{}: {:?} bytes", path, x.unwrap());
}
