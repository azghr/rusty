use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

// Only even numbers can be converted into `EvenNumber`.
impl TryFrom<i32> for EvenNumber {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err("only even numbers are allowed")
        }
    }
}

fn main() {
    // `TryFrom` returns a `Result`.
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(
        EvenNumber::try_from(5),
        Err("only even numbers are allowed")
    );

    // `TryInto` is automatically available because `TryFrom` is implemented.
    let result: Result<EvenNumber, _> = 10.try_into();
    println!("{:?}", result);

    let result: Result<EvenNumber, _> = 7.try_into();
    println!("{:?}", result);
}
