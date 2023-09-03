fn main(){
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Hash Maps and Ownership
    // For types that implement the copy trait like i32 the values are copied into the hash map.
    // For owned values like String the values will be moved and the hash map will be the owner of those values.

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();

    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, they were moved into the hash map.

    // Overwriting a Value
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // Adding only if it isn't present
    scores.entry(String::from("Red")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // Updating a value based on the old value
    let text = "hello world wonderful world";

    let mut text_map = HashMap::new();

    for word in text.split_whitespace(){
        let count = text_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", text_map);  // Recall that iterating over a hash map happens in an arbitrary order.
}