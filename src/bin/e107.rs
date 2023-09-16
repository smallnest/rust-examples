// Idiom #107 Get folder containing current program
// Assign to string dir the path of the folder containing the currently running executable.
// (This is not necessarily the working directory, though.)

fn main() -> std::io::Result<()>{
    let dir = std::env::current_exe()?
    .canonicalize()
    .expect("the current exe should exist");

    let d = dir.parent()
    .expect("the current exe should be a file")
    .to_string_lossy()
    .to_owned();

    println!("{}", d);

    Ok(())
}