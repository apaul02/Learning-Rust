fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("the config is {max}");
    }

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
   }
}

fn check(x: &u32) -> bool {
    match x {
        1 => true,
        2 => false,
        3 => true,
        _ => false
        
    }
}
#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
    
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for america"))
        }else {
            Some(format!("{state:?} is pretty new for america"))
        }
    }else {
        None
    }
}

fn describe_state_quarter2(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old for america"))

    }else {
        Some(format!("{state:?} is pretty new for america"))
    }
}
