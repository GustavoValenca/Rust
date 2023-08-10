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

    println!("{:?}", v);
}