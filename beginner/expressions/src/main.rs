fn main() {
    let x = 5u32;

    // Block returns its last expression
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        x_cube + x_squared + x
    };

    // Semicolon suppresses the return value
    let z = {
        let w = 2 * x;
        w
    };

    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {:?}", z);
}
