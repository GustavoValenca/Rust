// This code won't compile because rust doesn't allow null values.
// We are trying to use the variable r even though no value is associed to it.
// fn main(){
//     let r;

//     {
//         let x = 5;
//         r = &x;
//     }

//     println!("r: {}", r);
// }

fn main(){
    // let string1 = String::from("abcd");
    // let string2 = "xyz";

    // let result = longest(string1.as_str(), string2);
    // println!("The longest string is {}", result);

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len(){
        x
    } else {
        y
    }
}