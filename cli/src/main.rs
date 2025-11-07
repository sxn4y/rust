use std::io::stdin;
use colored::Colorize;

fn main() {
    loop {
        let line_prefix = "#-".color(rand_color());
        stdin()
            .read_line(&mut String::new())
            .expect("Failed to read line");
        println!("{line_prefix} ");
        
    }
}

fn rand_color() -> colored::Color {
    use colored::Color::*;
    use rand::Rng;

    let colors = [
        Red, Green, Yellow, Blue, Magenta, Cyan, White,
        BrightRed, BrightGreen, BrightYellow, BrightBlue,
        BrightMagenta, BrightCyan, BrightWhite,
    ];

    let mut rng = rand::rng();
    let idx = rng.random_range(0..colors.len());
    colors[idx]
}
