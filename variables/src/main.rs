fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}

fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    println!("f64: {}", type_of(&x));
    println!("f32: {}", type_of(&y));
}
