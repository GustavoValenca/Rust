use std::collections::HashMap;
use std::io;
use std::process;

fn main(){
    let mut deps: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        menu();
        let mut option = String::new();
        get_string_input(&mut option);
        option = capitalize(&option);
        println!("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-");


        if option == "A" {
            add_employee(&mut deps);
        }
        else if option == "D" {
            list_department(&mut deps);
        }
        else if option == "C" {
            println!("Company Employees:");
            println!("{:?}", deps);
        }
        else if option == "X" {
            println!("Exited successfully.");
            process::exit(0);
        }
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn menu(){
    println!("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-");
    println!("(A): Add new employee to a department");
    println!("(D): List all people in a department");
    println!("(C): List all people in the company");
    println!("(X): Exit");
    println!("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-");
}

fn get_string_input(reference: &mut String){
    io::stdin()
        .read_line(reference)
        .expect("Failed to read line");
    reference.pop();
}

fn add_employee(deps: &mut HashMap<String, Vec<String>>){
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

fn list_department(deps: &mut HashMap<String, Vec<String>>){
    let mut d = String::new();
    println!("Enter the Department: ");
    get_string_input(&mut d);
    d = capitalize(&d);

    let v = deps.get(&d);

    match v {
        Some(vector) => {
            println!("{:?}", vector);
        },
        None => {
            println!("Invalid department.");
        }
    }
}
