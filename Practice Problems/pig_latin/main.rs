use std::io;

fn main(){
    let mut string = String::new();

    io::stdin()
        .read_line(&mut string)
        .expect("Failed to read line");

    string = String::from(string.trim());

    let string_pig_latin = pig_latin(&string);

    println!("");
    println!("The text in pig latin: ");
    println!("{}", string_pig_latin);
}

fn pig_latin(string: &String) -> String {
    let mut new_string = String::new();

    let words = string.split(" ");
    
    let mut result;
    let mut complement;
    let mut counter = 0;
    for word in words {
        let word = word.split_at(1);
        if is_vogal(&word.0){
            complement = "-hay";
            result = [word.0, word.1, complement].join("");
        }
        else {
            let complement1 = "-";
            let complement2 = "ay";

            result = [word.1, complement1, word.0, complement2].join("");
        }
        
        println!("{}", result);

        if counter != 0 {
            new_string.push_str(" ");
        }
        new_string.push_str(&result);
        counter += 1;
    }

    return new_string;
}

fn is_vogal(character: &&str) -> bool {
    let c = *character;
    if c == "a" || c == "e" || c == "i" || c == "o" || c == "u" {
        return true;
    }
    else {
        return false;
    }
}