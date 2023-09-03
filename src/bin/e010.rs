// Idiom #10 Shuffle a list
// Generate a random permutation of the elements of list x

use rand::seq::SliceRandom;
use rand::thread_rng;
 
fn main() {
    let mut x = [1, 2, 3, 4, 5];
    println!("Unshuffled: {:?}", x);

    let mut rng = thread_rng();
    x.shuffle(&mut rng);

    println!("Shuffled:   {:?}", x);
}
