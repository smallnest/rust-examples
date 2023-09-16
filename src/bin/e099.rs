// Idiom #99 Format date YYYY-MM-DD
// Assign to the string x the value of the fields (year, month, day) of the date d, in format YYYY-MM-DD.

use chrono::*;

fn main() {
    println!("{}", Utc::now().format("%Y-%m-%d"))
}
