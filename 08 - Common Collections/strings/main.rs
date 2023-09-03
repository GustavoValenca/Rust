fn main(){
    let mut s = String::new();

    let data = "initial contents";

    let s1 = data.to_string();

    let s1 = "initial contents".to_string();

    let s1 = String::from("initial contents");

    // In this case, String::from and to_string do the same thing, so which you choose is a matter of style and readability.

    let mut s = String::from("foo");
    s.push_str("bar");  // takes a string slice
    println!("{s}");

    s.push('f');  // takes a single character
    println!("{s}");

    // Concatenating 2 strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // Concatenating more than 2 strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{s}");

    let s1 = String::from("tic");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    // Iterating over strings 
    
    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}