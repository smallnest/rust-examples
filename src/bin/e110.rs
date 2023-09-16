// Idiom #111 Launch other program
// From current process, run program x with command-line parameters "a", "b".

use std::process::Command;

fn main() {
    let status = Command::new("ls")
        .args(&["/etc"])
        .status()
        .expect("failed to execute process");

    // exit code is outputted after _ls_ runs
    println!("{}", status);

    let output = Command::new("ls")
        .args(&["/etc"])
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8(output.stdout).unwrap();

    println!("{}", output);
}
