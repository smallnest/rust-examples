// Idiom #5 Create a 2D Point data structure
// Declare a container type for two floating-point numbers x and y
use std::fmt;

struct Point {
    x: f64,
    y: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 2.0, y: -3.5 };

    println!("{}", p);
}
