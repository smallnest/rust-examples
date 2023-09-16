// Idiom #88 Allocate 1M bytes
// Create a new bytes buffer buf of size 1,000,000.

fn main() {
    let buf: Vec<u8> = Vec::with_capacity(1024 * 1024);
    println!("{:?}", buf.capacity());
}