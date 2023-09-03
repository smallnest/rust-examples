// Idiom #14 Pick uniformly a random floating point number in [a..b)
// Pick a random number greater than or equals to a, strictly inferior to b. Precondition : a < b.


use rand::{thread_rng, Rng};

fn main() {
    let (a, b) = (1.0, 3.0);
    let c = thread_rng().gen_range(a..b);
    println!("{}", c);
}
