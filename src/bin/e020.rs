// Idiom #20 Return two values
// Implement a function search which looks for item x in a 2D matrix m.
// Return indices i, j of the matching cell.
// Think of the most idiomatic way in the language to return the two values at the same time.


fn search<T: Eq>(m: &Vec<Vec<T>>, x: &T) -> Option<(usize, usize)> {
    for (i, row) in m.iter().enumerate() {
        for (j, column) in row.iter().enumerate() {
            if *column == *x {
                return Some((i, j));
            }
        }
    }
    
    None
}

fn main() {
    let a = vec![
        vec![0, 11],
        vec![22, 33],
        vec![44, 55],
    ];
    
    let hit = search(&a, &33);
    
    println!("{:?}", hit);
}