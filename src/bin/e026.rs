// Idiom #26 Create a 2-dimensional array
// Declare and initialize a matrix x having m rows and n columns, containing real numbers.

fn main() {
    const M: usize = 3;
    const N: usize = 4;
  
    let mut x = [[0.0; N] ; M];
    x[1][3] = 5.0;
    println!("{:#?}", x);

    let x = vec![vec![0.0f64; N]; M];    
    println!("{:#?}", x);

  }
  