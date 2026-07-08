fn main() {
    // Because of the suffix, the compiler knows that `elem` has type `u8`.
    let elem = 5u8;

    // Create an empty, growable vector.
    let mut vec = Vec::new();

    // At this point, the compiler only knows that `vec` is a `Vec<_>`.
    // The element type has not yet been inferred.

    // Push `elem` into the vector.
    vec.push(elem);

    // Now the compiler can infer that `vec` has type `Vec<u8>`.
    // Try commenting out the line above to see type inference fail.
    // (The compiler can no longer determine the element type.)

    println!("vec = {:?}", vec);
}
