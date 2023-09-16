// Idiom #89 Handle invalid argument
// You've detected that the integer value of argument x passed to the current function is invalid. Write the idiomatic way to abort the function execution and signal the problem.

#[derive(Debug, PartialEq, Eq)]
enum CustomError { InvalidAnswer }

fn do_stuff(x: i32) -> Result<i32, CustomError> {
    if x != 42 {
        Err(CustomError::InvalidAnswer)
    } else {
        Ok(x)
    }
}

fn main() {
    let x = 41;
    let result = do_stuff(x);
    match result {
        Ok(x) => println!("Result: {}", x),
        Err(e) => panic!("Error: {:?}", e),
    }
}