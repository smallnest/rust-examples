// Idiom #105 Current executable name
// Assign to the string s the name of the currently executing program (but not its full path).

fn main() {
    let s = std::env::current_exe()
        .expect("Can't get the exec path")
        .file_name()
        .expect("Can't get the exec name")
        .to_string_lossy()
        .into_owned();
    
    println!("{}", s);
}