// Idiom #27 Create a 3-dimensional array
// Declare and initialize a 3D array x, having dimensions boundaries m, n, p, and containing real numbers.

fn main() {
    const M: usize = 4;
    const N: usize = 6;
    const P: usize = 2;

    let x = [[[0.0f64; P]; N]; M];
    println!("{:#?}", x);


    let x = vec![vec![vec![0.0f64; P]; N]; M];
    println!("{:#?}", x);
}
