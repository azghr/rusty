fn main() {
    let pi = 3.14159265359999999999999999999999999999; // default f64
    let pi_32: f32 = 3.14159279999999999999999999999999999999;

    println!("pi: {}", pi); // pi: 3.1415926536
    println!("pi_32: {}", pi_32); // pi_32: 3.1415927

    let nan = f64::NAN;
    let inf = f64::INFINITY;
    let neg_inf = f64::NEG_INFINITY;

    println!("nan: {}", nan);
    println!("inf: {}", inf);
    println!("neg_inf: {}", neg_inf);

    let a = 0.1;
    let b = 0.2;
    let sum = a + b;

    println!("sum: {}", sum); // sum: 0.30000000000000004
    println!("{} + {} = {}", a, b, sum); // 0.1 + 0.2 = 0.30000000000000004
    println!("{} + {} == 0.3? {}", a, b, sum == 0.3); // 0.1 + 0.2 == 0.3? false
}
