// Idiom #11 Pick a random element from a list
// The list x must be non-empty.

use rand::seq::SliceRandom;
 
fn main() {
    let x = vec![11, 22, 33];

    let mut rng = rand::thread_rng();
    let choice = x.choose(&mut rng).unwrap();
    
    println!("I picked {}!", choice);
}
