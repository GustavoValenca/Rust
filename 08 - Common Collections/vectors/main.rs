use std::any::type_name;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main(){
    let v: Vec<i32> = Vec::new();

    let v1 = vec![1, 2, 3];

    println!("{}", type_of(v1));

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v2 = vec![1, 9, 3, 4, 5];

    let third: &i32 = &v2[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v2.get(2);

    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v3 = vec![100, 32, 57];
    for i in &v3 {
        println!("{i}");
    }

    let mut v4 = vec![100, 32, 57];
    for i in &mut v4 {
        *i += 50;
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue"));
        SpreadsheetCell::Float(10.12),
    ];

    // A vector is freed when it goes out of scope
    {
        let v5 = vec![1, 2, 3, 4];
        // do stuff with v5
    } // <- v goes out of scope and is freed here
}
