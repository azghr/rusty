fn main() {
    // Suffixed literals: the type is explicitly specified.
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literals: the compiler infers the type from context.
    // Here, `i` defaults to `i32` and `f` defaults to `f64`.
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a value in bytes.
    // std::mem::size_of_val is a function, but called with its full path.
    println!("size of `x` (u8)  : {} byte(s)", std::mem::size_of_val(&x));
    println!("size of `y` (u32) : {} byte(s)", std::mem::size_of_val(&y));
    println!("size of `z` (f32) : {} byte(s)", std::mem::size_of_val(&z));
    println!("size of `i` (i32) : {} byte(s)", std::mem::size_of_val(&i));
    println!("size of `f` (f64) : {} byte(s)", std::mem::size_of_val(&f));
}
