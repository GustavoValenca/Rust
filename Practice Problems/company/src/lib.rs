use std::collections::HashMap;
use std::io;

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn menu(){
    println!("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-");
    println!("(A): Add new employee to a department");
    println!("(D): List all people in a department");
    println!("(C): List all people in the company");
    println!("(X): Exit");
    println!("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-");
}

pub fn get_string_input(reference: &mut String){
    io::stdin()
        .read_line(reference)
        .expect("Failed to read line");
    reference.pop();
}

pub fn add_employee(deps: &mut HashMap<String, Vec<String>>){
    let mut d = String::new();

    println!("Enter the Department:");
    get_string_input(&mut d);
    
    d = capitalize(&d);

    let v = deps.contains_key(&d);

    let mut e = String::new();

    println!("Enter the Employee name:");
    get_string_input(&mut e);

    e = capitalize(&e);

    if v == false {
        let vector: &mut Vec<String> = &mut Vec::new();
        vector.push(e);
        deps.insert(d, vector.to_vec());
    }
    else {
        let vector = deps.get_key_value(&d);
        let (_k, vec): (&String, & Vec<String>) = vector.unwrap();
        let mut new_vec = vec.clone();
        new_vec.push(e);
        deps.insert(d, new_vec);
    }
}

pub fn list_department(deps: &mut HashMap<String, Vec<String>>){
    let mut d = String::new();
    println!("Enter the Department: ");
    get_string_input(&mut d);
    d = capitalize(&d);

    let v = deps.get(&d);

    match v {
        Some(vector) => {
            list_employees(vector);
        },
        None => {
            println!("Invalid department.");
        }
    }
}

pub fn list_company(deps: &mut HashMap<String, Vec<String>>){
    for (key, value) in deps {
        println!("{key}");
        list_employees(value);
    }
}

pub fn list_employees(vec: &Vec<String>){
    let mut v = vec.clone();
    v.sort();
    println!("{:?}", v);
}