// Idiom #106 Get program working directory
// Assign to string dir the path of the working directory.
// (This is not necessarily the folder containing the executable itself)


use std::env;

fn main() {
    let dir = env::current_dir().unwrap();

    println!("{:?}", dir);
}
