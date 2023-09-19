use rand::Rng;

fn main() {
    let mut v: Vec<i32> = Vec::new();

    let mut counter = 0;
    loop {
        counter += 1;
        let x = rand::thread_rng().gen_range(1..=100);

        v.push(x);

        if counter == 13 {
            break;
        }
    }

    println!("Original vector: ");
    println!("{:?}", v);

    let mut key : i32;
    let mut j : usize;
    let mut j_signed : i32;

    for i in 1..13 {

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

        println!("After execution {i}");
        println!("{:?}", v);
    }
}

