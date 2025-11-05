fn main() {
    let sum = my_function(32, -32);
    println!("The sum is: {}", sum);
}

fn my_function(x: i32, y: i32) -> i32 {
    println!("x = {}\ny = {}", x, y);
    x+y
}
