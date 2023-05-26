use std::io;

fn main() {
    loop {
        println!("Enter a integer n to get the nth fibonacci value: ");
        println!("Enter X to exit the program");
        println!("--------------------------------------------------");
        
        let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line.");

        if n.trim().to_uppercase() == "X" {
            break;
        }

        let n: u32 = match n.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Failed to read integer. Try again:");
                continue;
            },
        };

        if n == 0 {
            println!("The fibonacci value is 0");
            println!("--------------------------");
            continue;
        }
        else if n == 1 {
            println!("The fibonacci value is 1");
            println!("--------------------------");
            continue;
        }

        let mut before_last = 1;
        let mut last = 1;
        let mut i = 2;
        let mut current = 1;
        while i < n {
            current = last + before_last;
            before_last = last;
            last = current;

            i += 1;
        }

        println!("The fibonacci value is {}", current);
        println!("--------------------------");
    }

    println!("Exit success.")
}
