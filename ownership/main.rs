fn main() {
    // Variable Scope

    let s = "hi";
    println!("The value of s is {s}");

    {
        let s = "hello";  
        println!("The value of s is {s}");
    }

    println!("The value of s is {s}");
}
