// Idiom #94 Print the type of a variable
// Print the name of the type of x. Explain if it is a static type or dynamic type.

// This may not make sense in all languages.


#![feature(core_intrinsics)]

fn type_of<T>(_: &T) -> String {
    format!("{}", std::intrinsics::type_name::<T>())
}

fn main() {
    let x: i32 = 1;
    println!("{}", type_of(&x));
}