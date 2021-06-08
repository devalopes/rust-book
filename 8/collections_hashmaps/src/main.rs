use std::collections::HashMap;


fn main() {
    // All keys & values must have same type
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 30);
    scores.insert(String::from("Red"), 10);


    // Can construct a hash map via iterators
    let teams = vec![String::from("Blue"), String::from("Red")];
    let points = vec![30, 10];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(points.into_iter()).collect();

    let team_name = String::from("Blue");
    println!("{:?}", scores.get(&team_name));

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }


    // Overwriting value
    scores.insert(String::from("Blue"), 10);
    println!("{:?}", scores.get(&team_name));


    // Only inserting if key has no value
    scores.entry(String::from("Blue")).or_insert(50);


    // Look up keys value and update it
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);



}
