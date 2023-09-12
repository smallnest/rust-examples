// Idiom #44 Insert element in list
// Insert the element x at position i in the list s. Further elements must be shifted to the right.

fn main() {
    let mut vec = vec![1, 2, 3];
    vec.insert(1, 4);
    assert_eq!(vec, [1, 4, 2, 3]);
    vec.insert(4, 5);
    assert_eq!(vec, [1, 4, 2, 3, 5]);
 
    println!("{:?}", vec);
}