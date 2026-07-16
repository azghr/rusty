fn main() {
    // let mut data: [i32; 5] = [1, 2, 3, 4, 5];
    let mut data = [1, 2, 3, 4, 5];
    println!("data={:?}", data);

    // updating elements
    data[0] = 10;
    data[1] = 20;
    data[2] = 30;
    data[3] = 40;
    data[4] = 50;
    println!("data={:?}", data);

    // accessing elements
    println!("Element at index 0: {}", data[0]);
    println!("Element at index 1: {}", data[1]);
    println!("Element at index 2: {}", data[2]);
    println!("Element at index 3: {}", data[3]);
    println!("Element at index 4: {}", data[4]);

    // using for loop
    for item in data {
        println!("item: {}", item);
    }

    // repeat
    let repeat = ["bob"; 10];
    println!("repeat={:?}", repeat);
}
