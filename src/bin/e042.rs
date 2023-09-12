// Idiom #42 Continue outer loop
// Print each item v of list a which is not contained in list b.
// For this, write an outer loop to iterate on a and an inner loop to iterate on b.

fn main() {
    let a = ['a', 'b', 'c', 'd', 'e'];
    let b = [     'b',      'd'     ];
    
    'outer: for va in &a {
        for vb in &b {
            if va == vb {
                continue 'outer;
            }
        }
        println!("{}", va);
    }
}
