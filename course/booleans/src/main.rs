fn main() {
    let connected_to_internet: bool = false;
    println!("Connected to internet: {}", connected_to_internet);

    let has_cat: bool = true;
    println!("User has cat: {}", has_cat);

    let money = 5_000;
    println!("money > 0 = {}", money > 0); // money > 0 = true

    if money > 0 {
        println!("You are not broke");
    }
}
