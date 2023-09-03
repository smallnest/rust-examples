// Idiom #15 Pick uniformly a random integer in [a..b]
// Pick a random integer greater than or equals to a, inferior or equals to b. Precondition : a < b.
use rand::distributions::Distribution;
use rand::distributions::Uniform;
fn main() {
    let (a, b) = (3, 5);

    let x = Uniform::new_inclusive(a, b).sample(&mut rand::thread_rng());

    println!("{}", x);
}