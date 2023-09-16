// Idiom #109 Number of bytes of a type
// Set n to the number of bytes of a variable t (of type T).

// T has (8 + 4) == 12 bytes of data
struct T(f64, i32);

fn main() {
    let n = ::std::mem::size_of::<T>();

    println!("{} bytes", n);
    // T has size 16, which is "the offset in bytes between successive elements in an array with item type T"
}
