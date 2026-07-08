// Suppress warnings about overflowing literals used in examples.
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // Error! Rust does not perform implicit conversions.
    // let integer: u8 = decimal;

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    // Error! A float cannot be directly converted to a char.
    // let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // When casting to an unsigned integer type, the value is truncated
    // (only the least significant bits are kept) if it does not fit.

    // 1000 already fits in a u16.
    println!("1000 as a u16 is : {}", 1000 as u16);

    // 1000 -> 232 (1000 % 256).
    println!("1000 as a u8  is : {}", 1000 as u8);

    // -1 -> 255.
    println!("  -1 as a u8  is : {}", (-1i8) as u8);

    // For positive numbers, truncation is equivalent to modulo.
    println!("1000 mod 256  is : {}", 1000 % 256);

    // Casting to signed integers preserves the bit pattern.
    println!(" 128 as a i16 is : {}", 128 as i16);

    // 128 in 8-bit two's complement is -128.
    println!(" 128 as a i8  is : {}", 128 as i8);

    // 1000 as u8 -> 232.
    println!("1000 as a u8  is : {}", 1000 as u8);

    // 232 interpreted as i8 is -24.
    println!(" 232 as a i8  is : {}", 232 as i8);

    // Since Rust 1.45, float-to-int casts are saturating.

    // Values above the maximum become the maximum.
    println!(" 300.0 as u8 is : {}", 300.0_f32 as u8);

    // Values below the minimum become the minimum.
    println!("-100.0 as u8 is : {}", -100.0_f32 as u8);

    // NaN converts to 0.
    println!("   NaN as u8 is : {}", f32::NAN as u8);

    // Unsafe conversion skips saturation checks and may overflow.
    unsafe {
        println!(
            " 300.0 unchecked u8 is : {}",
            300.0_f32.to_int_unchecked::<u8>()
        );

        println!(
            "-100.0 unchecked u8 is : {}",
            (-100.0_f32).to_int_unchecked::<u8>()
        );

        println!(
            "   NaN unchecked u8 is : {}",
            f32::NAN.to_int_unchecked::<u8>()
        );
    }
}
