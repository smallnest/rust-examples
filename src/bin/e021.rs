// Idiom #21 Swap values
// Swap the values of the variables a and b

fn main() {
    let (mut a, mut b) = (12, 42);
    println!("a = {}, b = {}", a, b);
    
    std::mem::swap(&mut a, &mut b);
    println!("a = {}, b = {}", a, b);

    let (a, b) = (b, a);
     println!("a = {}, b = {}", a, b);
}