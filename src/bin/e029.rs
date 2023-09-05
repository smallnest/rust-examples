// Idiom #29 Remove item from list, by its index
// Remove i-th item from list items.
// This will alter the original list or return a new list, depending on which is more idiomatic.
// Note that in most languages, the smallest valid value for i is 0.


fn main() {
    let mut v = vec![1, 2, 3];
    assert_eq!(v.remove(1), 2);
    assert_eq!(v, [1, 3]);
    
}