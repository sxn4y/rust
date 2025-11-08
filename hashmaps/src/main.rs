use std::collections::HashMap;

fn main() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let team_name = String::from("Blue");
    let score = match scores.get(&team_name) {
        Some(&score) => score,
        None => 0,
    };

    println!("{:#?}", scores);
}
