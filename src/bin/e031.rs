// Idiom #31 Recursive factorial (simple)
// Create the recursive function f which returns the factorial of the non-negative integer i, calculated from f(i-1)

fn factorial(num: u64) -> u64 {
    match num {
        0 | 1=> 1,
        _ => factorial(num - 1) * num,
    }
}

fn main (){
    println!("{}", factorial(0));
    println!("{}", factorial(1));
    println!("{}", factorial(2));
    println!("{}", factorial(3));
    println!("{}", factorial(4));
    println!("{}", factorial(5));
}

