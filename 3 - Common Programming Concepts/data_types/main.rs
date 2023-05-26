fn main() {
    let x = 2.0;  // f64
    let y: f32 = 3.0;  // f32

    // Numeric Operations

    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;  // Results -1

    let remainder = 43 % 5;

    println!("{sum} {difference} {product} {quotient} {truncated} {remainder}");

    // Boolean Type

    let t = true;
    let f: bool = false;

    // Character Type

    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';

    // ^ We use single quotes for char literals
    // Char type is four bytes in size

    // Tuple

    let tup: (i32, f64, u8) = (500, 6.4, 1);  // Optional type annotation

    let (x, y, mut z) = tup;  // destructuring

    println!("The value of y is {y}, x: {x}, z:{z}");

    z = 10;
    println!("z:{z}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("Primeiro: {five_hundred} {{}}");  // escape character

    // Array

    let a = [1, 2, 3, 4, 5];  // Data allocated on the stack rather than the heap

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5]; // The same as writing "let a = [3, 3, 3, 3, 3]"

    let mut a = [1, 2, 3, 4, 5];

    let first = a[0];
    a[1] = 40;

    let mut second = a[1];

    println!("First: {first}");
    println!("Second: {second}");

    second = 59;
    println!("Second modificado: {second}");
    let second = a[1];
    println!("Second modificado: {second}");
}
