// Idiom #70 Use clock as random generator seed
// Get the current datetime and provide it as a seed to a random generator. The generator sequence will be different at each run.

use rand::{Rng, SeedableRng, rngs::StdRng};
use std::time::SystemTime;

fn main() {
    let d = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Duration since UNIX_EPOCH failed");
    let mut rng = StdRng::seed_from_u64(d.as_secs());
    
    println!("{:?}", rng.gen::<f32>());
}