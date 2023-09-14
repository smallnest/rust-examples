// Idiom #54 Compute sum of integers
// Calculate the sum s of the integer list or array x.

fn main() {
    let x: Vec<usize> = (0..=10_000).collect();
    
    eprintln!("Sum of 0-10,000 = {}", x.iter().sum::<usize>())
}