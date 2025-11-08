fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{}, {}, {}", five.unwrap_or(0), six.unwrap_or(0), none.unwrap_or(0));
    
    // use match for exhaustive patterns

    // match six {
    //     Some(6) => println!("six")
    //     _ => (), // this line is extra for this specific use-case
    // }
    
    // use if let for non-exhaustive patterns

    // if let Some(6) = six {
    //     println!("six");
    // }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}
