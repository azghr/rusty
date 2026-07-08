fn main() {
    println!("=== Example 1: Freezing by Shadowing ===");

    let mut mutable_integer = 7;

    {
        // Shadow the mutable variable with an immutable one.
        let mutable_integer = mutable_integer;

        println!("Shadowed value: {}", mutable_integer);

        // Error: `mutable_integer` is immutable in this scope.
        // mutable_integer = 50;
    }

    // The original mutable variable is still accessible.
    mutable_integer = 3;
    println!("Original mutable value: {}", mutable_integer);

    println!("\n=== Example 2: Freezing by Immutable Borrow ===");

    let mut value = 7;

    {
        // Immutable borrow of `value`.
        let frozen = &value;

        println!("Frozen value: {}", frozen);

        // Error: Cannot modify `value` while it is immutably borrowed.
        // value = 10;
    } // `frozen` goes out of scope here.

    // Now the borrow has ended, so mutation is allowed.
    value = 10;

    println!("Updated value: {}", value);
}
