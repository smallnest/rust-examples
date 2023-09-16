// Idiom #78 "do while" loop
// Execute a block once, then execute it again as long as boolean condition c is true.

fn main() {
    let mut i = 5;
    loop {
        println!("{}", i);

        if i % 2 == 0 {
            i /= 2;
        } else {
            i = 3 * i + 1;
        }

        let c = i > 1;
        if !c {
            break;
        }
    }

    println!("And the final value: {}", i);


    let mut i = 5;
    while {
        println!("{}", i);

        if i % 2 == 0 {
            i /= 2;
        } else {
            i = 3 * i + 1;
        }

        let c = i > 1;
        c
    } { /* EMPTY */ }

    println!("And the final value: {}", i);
}
