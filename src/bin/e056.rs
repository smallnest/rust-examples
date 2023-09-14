// Idiom #56 Launch 1000 parallel tasks and wait for completion
// Fork-join : launch the concurrent execution of procedure f with parameter i from 1 to 1000.
// Tasks are independent and f(i) doesn't return any value.
// Tasks need not run all at the same time, so you may use a pool.
// Wait for the completion of the 1000 tasks and then print "Finished".

use std::thread;

fn f(i: i32) {
    i + 1;
}

fn main() {
    let threads: Vec<_> = (0..10).map(|i| thread::spawn(move || f(i))).collect();

    for t in threads {
    	t.join();
    }
}


