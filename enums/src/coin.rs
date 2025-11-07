fn main() {
    println!("Value of a dime: {} cents", value_in_cents(Coin::Dime, 1));
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin, amount: u32) -> u32 {
    match coin {
        Coin::Penny => 1 * amount,
        Coin::Nickel => 5 * amount,
        Coin::Dime => 10 * amount,
        Coin::Quarter => 25 * amount,
    }
}
