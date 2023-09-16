// Idiom #87 Stop program
// Exit immediately.
// If some extra cleanup work is executed by the program runtime (not by the OS itself), describe it.
#[allow(unreachable_code)]

fn main() {
    std::process::exit(1);
    
    println!("42");
}