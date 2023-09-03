struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main(){
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("{}", user1.username);

    let mut user1 = user1;  // We can't mark only certain fields as mutable. The entire instance must 
    user1.email = String::from("gustavo@gmail.com");
    println!("{}", user1.email);


    // New user without the update syntax
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // Using update syntax
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1  // The syntax .. specifies that the remaining fields not explicitly set should have the same values as the fields in the given instance
    };

    // We can't use user1 as whole because some values were moved
    println!("{}", user1.email);
    // println!("{}", user1.username);  // We can't use username because it was moved
    println!("{}", user1.sign_in_count);


    // Tuple Structs
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Note that black and origin values are different types because they're instances of different tuple structs.
    // A function that takes a parameter of type Color cannot take a Point as an argument, even though both types are made up of three i32 values.

    
    let orange = Color(255, 165, 0);

    println!("{}", orange.1);

    // Unit-like structs 
    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// Tuple structs

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like structs

struct AlwaysEqual;
