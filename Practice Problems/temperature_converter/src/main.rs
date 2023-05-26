use std::io;

fn main() {
    println!("Convert temperatures between Fahrenheit and Celsius!");
    println!("--------------------------------------------------------");
    
    loop {
        println!("Enter the temperature scale:");
        println!("F for Fahrenheit");
        println!("C for Celsius");
        println!("X to exit the program");
        println!("--------------------------------------------------------");
        
        let mut scale = String::new();

        io::stdin()
            .read_line(&mut scale)
            .expect("Failed to read line.");

        scale = scale.trim().to_uppercase();

        if scale == "C" {
            loop {
                println!("--------------------------------------------"); 
                println!("What is the temperature in °C ?");
                println!("--------------------------------------------");

                let mut temperature = String::new();

                io::stdin()
                    .read_line(&mut temperature)
                    .expect("Failed to read line.");

                let temperature: f32 = match temperature.trim().parse(){
                    Ok(num) => num,
                    Err(_) => {
                        println!("Failed to read temperature. Try again:");
                        continue
                    },
                };

                let temperature_f = convert_to_fahrenheit(temperature);

                println!("--------------------------------------------");
                println!("The temperature in Fahrenheit é {:.1}°F", temperature_f);
                println!("--------------------------------------------");

                break;
            }
            continue;   
        }
        else if scale == "F" {
            loop {
                println!("--------------------------------------------"); 
                println!("Now what is the temperature in °F ?");
                println!("--------------------------------------------"); 
                
                let mut temperature = String::new();

                io::stdin()
                    .read_line(&mut temperature)
                    .expect("Failed to read line.");

                let temperature: f32 = match temperature.trim().parse(){
                    Ok(num) => num,
                    Err(_) => {
                        println!("Failed to read temperature. Try again:");
                        continue
                    }
                };

                let temperature_c = convert_to_celsius(temperature);

                println!("--------------------------------------------");
                println!("The temperature in Fahrenheit é {:.1}°F", temperature_c);
                println!("--------------------------------------------");
                break;
            }
        
            continue;
        }
        else if scale == "X" {
            break;
        }
        else {
            println!("Invalid option. Try again:")
        }
    }
    println!("Exit success.")
}

fn convert_to_fahrenheit(x: f32) -> f32 {
    x * (9.0 / 5.0) + 32.0
}

fn convert_to_celsius(x: f32) -> f32 {
    (x - 32.0) * (5.0 / 9.0)
}
