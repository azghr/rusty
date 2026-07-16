fn main() {
    // signed and unsigned integers
    let signed: i8 = 10; // -128 to 127
    let unsigned: u8 = 10; // 0 to 255
    println!("signed: {}, unsigned: {}", signed, unsigned);

    // overflow
    let signed_overflow: i8 = 127;
    let unsigned_overflow: u8 = 255;
    println!(
        "signed_overflow: {}, unsigned_overflow: {}",
        signed_overflow, unsigned_overflow
    );

    let n = 100; // default i32
    let n_32: i32 = 100;
    println!("n: {}, n_32: {}", n, n_32);

    let n_64: i64 = 100;
    let n_128: i128 = 100;
    let n_isize: isize = 100; // pointer size
    let n_usize: usize = 100; // pointer size
    println!(
        "n_64: {}, n_128: {}, n_isize: {}, n_usize: {}",
        n_64, n_128, n_isize, n_usize
    );

    // print max and min size as well
    println!("isize MAX: {}", isize::MAX);
    println!("isize MIN: {}", isize::MIN);

    println!("usize MAX: {}", usize::MAX);
    println!("usize MIN: {}", usize::MIN);

    // NOTE: we can do this with any type like i8, i32 and so on
    println!("i8 MAX: {}", i8::MAX);
    println!("i8 MIN: {}", i8::MIN);

    println!("u8 MAX: {}", u8::MAX);
    println!("u8 MIN: {}", u8::MIN);

    let mut n: u8 = 255;
    n += 10;
    println!("n: {}", n); // wrap around -> run cargo run -q --release and will get 9 instead of error

    let trillions: i64 = 1_000_000_000_000;
    println!("trillions: {}", trillions);
}
