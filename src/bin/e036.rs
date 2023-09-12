// Idiom #36 First-class function : generic composition
// Implement a function compose which returns composition function g ∘ f for any functions f and g having exactly 1 parameter.

fn compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
	move |x| g(f(x))
}

fn main() {
    let f = |x: u32| (x * 2) as i32;
    let g = |x: i32| (x + 1) as f32;
    let c = compose(f, g);
    
    println!("{}", c(2));
}