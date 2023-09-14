// Idiom #69 Seed random generator
// Use seed s to initialize a random generator.
// 
// If s is constant, the generator output will be the same each time the program runs. If s is based on the current value of the system clock, the generator output will be different each time.

use rand::{Rng, SeedableRng, rngs::StdRng};

fn main() {
    let s = 32;
    let mut rng = StdRng::seed_from_u64(s);
    
    println!("{:?}", rng.gen::<f32>());
}