// Idiom #33 Atomically read and update variable
// Assign to the variable x the new value f(x), making sure that no other thread may modify x between the read and the write.


use std::sync::Mutex;

fn f(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = Mutex::new(0);
    let mut x = x.lock().unwrap();
    *x = f(*x);
    
    println!("{:?}", *x);
}
