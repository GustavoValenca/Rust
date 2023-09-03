fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition was true");
    }
    else {
        println!("Condition was false");
    }

    // Using if in a let statement

    let condition = false;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}")
}
