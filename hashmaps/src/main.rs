use std::collections::HashMap;

// fn main() {
//     let blue = String::from("Blue");
//     let yellow = String::from("Yellow");

//     let mut scores = HashMap::new();

//     scores.insert(blue, 10);
//     scores.insert(yellow, 50);

//     // let team_name = String::from("Blue");
//     // let score = match scores.get(&team_name) {
//     //     Some(&score) => score,
//     //     None => 0,
//     // };

//     for (key, value) in &scores {
//         println!("{}: {}", key, value);
//     }

//     println!("{:#?}", scores);
// }

// fn main() {
//     let mut scores = HashMap::new();

//     scores.insert("Blue", 20);

//     // scores.insert("Blue", 10); // for forced mutation
//     scores.entry("Blue").or_insert(10); // inserts if key "Blue" does not have a value

//     println!("{:#?}", scores);
// }

fn main() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map);
}
