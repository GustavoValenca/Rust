// # Borrowing

// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// # Mutable References

// fn main(){
//     let mut s = String::from("hello");

//     change(&mut s);
// }

// fn change(some_string: &mut String){
//     some_string.push_str(", world");
// }

// # Data racing

// fn main(){
//     let mut s = String::from("hello");

//     {
//         let r1 = &mut s;
//     }  // r1 goes out of scope here, so we can make a new reference with no problems.

//     let r2 = &mut s;

// }

// # Mutable and Immutable references

fn main(){
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s;
    println!("{}", r3)
}

