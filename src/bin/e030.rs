// Idiom #30 Parallelize execution of 1000 independent tasks
// Launch the concurrent execution of procedure f with parameter i from 1 to 1000.
// Tasks are independent and f(i) doesn't return any value.
// Tasks need not run all at the same time, so you may use a pool.


use rayon::prelude::*;
use std::thread;

fn main() {
    (0..1000).into_par_iter().for_each(f);

    let threads: Vec<_> = (0..1000).map(|i| thread::spawn(move || f(i))).collect();
    for thread in threads {
        thread.join();
    }
}

fn f(i: i32) {
    println!("{}", i);
}