///////////////////////////////////// From
// use std::convert::From;

// #[allow(dead_code)]
// #[derive(Debug)]
// struct Number {
//     value: i32,
// }

// impl From<i32> for Number {
//     fn from(item: i32) -> Self {
//         Number { value: item }
//     }
// }

// fn main() {
//     let num = Number::from(30);
//     println!("My number is {:?}", num);
// }

///////////////////////////////////// Into
// use std::convert::Into;

// #[allow(dead_code)]
// #[derive(Debug)]
// struct Number {
//     value: i32,
// }

// impl Into<Number> for i32 {
//     fn into(self) -> Number {
//         Number { value: self }
//     }
// }

// fn main() {
//     let int = 5;
//     // Try removing the type annotation
//     let num: Number = int.into();
//     println!("My number is {:?}", num);
// }

///////////////////////////////////// From and Into are interchangeable
// use std::convert::From;

// #[allow(dead_code)]
// #[derive(Debug)]
// struct Number {
//     value: i32,
// }

// // Define `From`
// impl From<i32> for Number {
//     fn from(item: i32) -> Self {
//         Number { value: item }
//     }
// }

// fn main() {
//     let int = 5;
//     // use `Into`
//     let num: Number = int.into();
//     println!("My number is {:?}", num);
// }

// `From` allows a type to define how it is created from another type.
#[allow(dead_code)]
#[derive(Debug)]
struct Number {
    value: i32,
}

// Convert from `i32` to `Number`.
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    // Convert a string slice into a `String`.
    let my_str = "hello";
    let my_string = String::from(my_str);

    println!("String: {}", my_string);

    // Convert an `i32` into a `Number` using `From`.
    let num1 = Number::from(30);

    println!("My number is {:?}", num1);

    // Since `From<i32>` is implemented for `Number`,
    // `Into<Number>` is automatically available.
    let num2: Number = 5.into();

    println!("My number is {:?}", num2);
}
