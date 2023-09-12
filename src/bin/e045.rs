// Idiom #45 Pause execution for 5 seconds
// Sleep for 5 seconds in current thread, before proceeding with the next instructions.

use std::time::Duration;
use std::thread::sleep;

fn main() {
    println!("Executing f in 3s...");
    sleep(Duration::new(3, 0));
    f(42);
    println!("done!");
}

fn f(n: i64) {
  println!("f received {}", n)
}