fn main() {
    println!("Hello world");
    let random_coin = Coin::Quater;
    let value = value_in_cents(random_coin);
    println!("{}", value);
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
         },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater => 25,
    }
}
