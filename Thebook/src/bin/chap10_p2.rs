// Patterns That Bind to Values
#[derive(Debug)]  // will use this to inspect the state in a minute 
enum UsState { 
    Alabama,
    Alaska,
    //-more here-
}

enum Coin { 
    Penny, 
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 { 
    match coin { 
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state quarter from {state:?}!");
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Quarter(UsState::Alabama));
    value_in_cents(Coin::Quarter(UsState::Alaska));
}

