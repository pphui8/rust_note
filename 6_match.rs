#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime(i8),
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime(_) => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn match_num(x: i8) -> i8 {
    match x {
        3 => 4,
        4 => 5,
        _ => todo! {}
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}