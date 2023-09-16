// Idiom #73 Create a factory
// Create a factory named fact for any sub class of Parent and taking exactly one string str as constructor parameter.

use core::fmt::Debug;
use std::str::FromStr;
fn fact<Parent: std::str::FromStr>(str: String, _: Parent) -> Parent where <Parent as FromStr>::Err: Debug 
{
    return str.parse::<Parent>().unwrap();
}

fn main() {
    let x = fact::<i32>("123".to_string(), 0);
    println!("{}", x);
}