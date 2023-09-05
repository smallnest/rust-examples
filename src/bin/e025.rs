// Idiom #25 Send a value to another thread
// Share the string value "Alan" with an existing running process which will then display "Hello, Alan"

use std::thread;
use std::sync::mpsc::channel;

fn main() {
    let (send, recv) = channel();

    let handle = thread::spawn(move || loop {
        let msg = recv.recv().unwrap();
        println!("Hello, {:?}", msg);
    });

    send.send("Alan").unwrap();
    
    handle.join().unwrap();
}
