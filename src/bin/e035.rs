// Idiom #35 First-class function : compose
// Implement a function compose (A -> C) with parameters f (A -> B) and g (B -> C), which returns the composition function g ∘ f

fn compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
	move |x| g(f(x))
}

fn main() {
    let f = |x: u32| (x * 2) as i32;
    let g = |x: i32| (x + 1) as f32;
    let c = compose(f, g);
    
    println!("{}", c(2));
}
