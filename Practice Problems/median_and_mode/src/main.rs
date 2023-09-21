use std::io;
use rand::Rng;
use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();

    // Vector length
    println!("Enter the vector's length: ");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    
        let n: usize = n.trim().parse()
            .expect("Please type a number!");

    let mut counter = 0;
    loop {
        counter += 1;
        let x = rand::thread_rng().gen_range(1..=100);

        v.push(x);

        if counter == n {
            break;
        }
    }

    println!("Original vector: ");
    println!("{:?}", v);

    let mut key : i32;
    let mut j : usize;
    let mut j_signed : i32;

    for i in 1..n {

        key = v[i];
        
        j = i - 1;
        j_signed = j as i32;

        while v[j] > key && j_signed >= 0 {
            v[j + 1] = v[j];
            j_signed -= 1;
            if j_signed < 0 {
                break;
            }
            j -= 1;

        }
        if j_signed < 0 {
            v[0] = key;
        }
        else {
            v[j + 1] = key;
        }

        // println!("After execution {i}");
        // println!("{:?}", v);
    }

    println!("Sorted vector: ");
    println!("{:?}", v);

    let median = v[n / 2];
    println!("Median: {median}");

    let mut map = HashMap::new();

    for i in &v {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    println!("Frequency of values: ");
    println!("{:?}", map);

    let mut biggest = 1;
    for (_, value) in &map {
        if *value > biggest {
            biggest = *value;
        }
    }

    let mut mode: Vec<i32> = Vec::new();
    for (key, value) in &map {
        if *value == biggest {
            mode.push(**key);
        }
    }

    println!("Mode: ");
    println!("{:?}", mode);
}

