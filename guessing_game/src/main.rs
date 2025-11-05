use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::Colorize;

fn main() {
    let inp = io::stdin();
    let secret: u32 = rand::rng().random_range(1..=100);
    println!("Guess the number!");
    loop {
        println!("Please input your guess: ");

        let mut guess = String::new(); 

        inp
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green().bold());
                break;
            },
        }
    }
}