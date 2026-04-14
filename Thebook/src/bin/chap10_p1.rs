#[derive(Debug)]

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin = Coin::Penny;
    dbg!(value_in_cents(coin));
    let coin = Coin::Nickel;
    value_in_cents(coin);
    let coin = Coin::Dime;
    value_in_cents(coin);
    let coin = Coin::Quarter;
    value_in_cents(coin);
    
   
}


