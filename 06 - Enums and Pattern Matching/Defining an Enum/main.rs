// Using enum and struct

// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// Using just enum

// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// Why is it better?

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
// You wouldn't be able to do that using a struct

// The Standard Library Definition

// struct Ipv4Addr {
//     // --snip--
// }

// struct Ipv6Addr {
//     // --snip--
// }

// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }

// Another Example

enum Message {
    Quit, 
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self){
        // method body would be defined here
    }
}

// Option

enum Option<T> {
    None,
    Some(T),
}

fn main(){
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let loopback = IpAddr::V6(String::from("::1"));

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');
    
    let absent_number: Option<i32> = Option::None;  // We specify the type because Rust can't infer it by looking at None, unlike some_number and some_char in which it is possible to infer the type by looking at the Some variant.

    match some_number {
        Some(value) => {
            println!("O valor é {value}.");
        }
        None => {
            println!("Não tem valor.");
        }
    }
}

// fn route(ip_kind: IpAddrKind){

// }