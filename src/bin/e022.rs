// Idiom #22 Convert string to integer
// Extract the integer value i from its string representation s (in radix 10)

fn main() {
    let mut s = "123";
    let mut i = match s.parse::<i32>() {
        Ok(i) => i,
        Err(_e) => -1,
    };
    println!("{:?}", i);

    s = "12u3";
    i = match s.parse::<i32>() {
        Ok(i) => i,
        Err(_e) => -1,
    };
    println!("{:?}", i);

    let mut i: i32 = s.parse().unwrap_or(0);
    println!("{:?}", i);

    s = "12u3";
    i = s.parse().unwrap_or(0);
    println!("{:?}", i);
}
