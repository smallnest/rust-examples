// Idiom #98 Epoch seconds to date object
// Convert a timestamp ts (number of seconds in epoch-time) to a date with time d. E.g. 0 -> 1970-01-01 00:00:00


use chrono::*;

fn main() {
    let ts = 1451606400;
    let d = NaiveDateTime::from_timestamp_opt(ts, 0).unwrap();
    println!("{}", d);
}
