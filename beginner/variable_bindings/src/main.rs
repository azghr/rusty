fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // Copy `an_integer` into `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // Prefixing a variable with `_` tells the compiler it's okay if it's unused.
    let _unused_variable = 3u32;

    // This warning is also suppressed because of the leading underscore.
    let _noisy_unused_variable = 2u32;
}
