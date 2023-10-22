use std::collections::HashMap;
use std::process;

use company::*;

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
            list_company(&mut deps);
        }
        else if option == "X" {
            println!("Exited successfully.");
            process::exit(0);
        }
    }
}
