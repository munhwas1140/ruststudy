fn main() {
    let number = 3;

    if number != 0 {
        println!("number is not zero");
    }

    let condition: bool = true;

    let number: i32 = if condition { 5 } else { 6 };
    println!("number is {number}")
}