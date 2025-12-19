#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel(UsState),
    Dime,
    Quarter,
}

fn _value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel(state) => {
            println!("Nickel {:?},", state);
            5
        }
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let config_max = Some(3u8);

    // match config_max {
    //     Some(max) => println!("The maximum is configure to be {max}"),
    //     _ => (),
    // }

    if let Some(max) = config_max {
        println!("The maximum is configure to be {max}");
    }

    println!("{:?}", config_max);

    let state = UsState::Alabama;

    let coin = Coin::Nickel(state);

    let mut count = 0;

    // match coin {
    //     Coin::Nickel(state) => println!("State nickel from {state:?}!"),
    //     _ => count += 1,
    // }

    if let Coin::Nickel(state) = coin {
        println!("State nickel from {state:?}!");
    } else {
        count += 1;
    }
}

fn _plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None,
    }
}

fn _describe_state_nikel(coin: Coin) -> Option<String> {
    if let Coin::Nickel(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}

fn _describe_state_nikel(coin: Coin) -> Option<String> {
    let state = if let Coin::Nickel(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn describe_state_nikel_with_early_return(coin: Coin) -> Option<String> {
    let Coin::Nickel(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
