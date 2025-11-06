use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("err");
    let n: u16 = n.trim().parse().unwrap();
    let mut M: [[char; n]; n] = [[' '; n]; n];
}
