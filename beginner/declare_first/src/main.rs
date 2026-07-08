fn main() {
    // Declare the variable without initializing it.
    // Rust allows this as long as it's initialized before use.
    let a_binding;

    {
        let x = 2;

        // Initialize the variable inside the inner scope.
        a_binding = x * x;
    } // `x` goes out of scope here.

    println!("a_binding: {}", a_binding);

    // Declare another variable.
    let another_binding;

    // Uncommenting the line below would cause a compile-time error,
    // because `another_binding` has not been initialized yet.
    //
    // println!("another_binding: {}", another_binding);

    // Initialize the variable before using it.
    another_binding = 1;

    println!("another_binding: {}", another_binding);
}
