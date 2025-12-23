enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let quarter = Coin::Dime;

    if let Coin::Quarter = quarter {
        println!("Quarter coin");
    } else {
        println!("Not is Quarter coin");
    }

    let Coin::Quarter = quarter else {
        return println!("Not is Quarter coin");
    };
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}