#[allow(unused)]
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[allow(unused)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!("The value of a Dime is: {}", value_in_cents(Coin::Dime));
    println!("The value of a Penny is: {}", value_in_cents(Coin::Penny));
    println!("The value of a Penny is: {}", value_in_cents(Coin::Quarter(UsState::Alaska)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{five:?}");
    println!("{six:?}");
    println!("{none:?}");

    let some_number = 10;

    match some_number {
        1 => println!("number 1"),
        2 => println!("number 2"),
        _ => ()
    }

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be: {max}"),
        _ => ()
    }

    let config_max = Some(32u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be: {max}");
    }
}
