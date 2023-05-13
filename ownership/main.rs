fn main() {
    // Variable Scope

    let s = "hi";
    println!("The value of s is {s}");

    {
        let s = "hello";  
        println!("The value of s is {s}");
    }

    println!("The value of s is {s}");

    // The String Type

    let mut s = String::from("hello");
    
    s.push_str(", world!");  // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`   

    // Move

    let s1 = String::from("Hello");
    let s2 = s1;

    // Clone

    let s1 = String::from("hello");
    let s2 = s1.clone();

    // Ownership and Functions

    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

    // Return Values and Scope

    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    println!("{}", s3);

    // Return multiple values using a tuple

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len)
}


fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize){
    let length = s.len();

    (s, length)
}