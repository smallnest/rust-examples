// Idiom #43 Break outer loop
// Look for a negative value v in 2D integer matrix m. Print it and stop searching.

fn main() {
    let m = vec![
        vec![1, 2, 3],
        vec![11, 0, 30],
        vec![5, -20, 55],
        vec![0, 0, -60],
    ];
    
    'outer: for v in m {
        'inner: for i in v {
            if i < 0 {
                println!("Found {}", i);
                break 'outer;
            }
        }
    }
}