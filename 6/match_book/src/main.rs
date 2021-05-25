#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    //...
    NewJersey,
    NewYork,
    //...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin)-> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}


// Matching with Option<T> (rusts way of handling nulls)
// Need to convert Option<T> -> T to be able to use the T
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x{
        None => None,
        Some(i) => Some(i + 1)
    }
}


fn main() {
    println!("Value in cents: {}", value_in_cents(Coin::Quarter(UsState::NewJersey)));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    // Handling `None` in match
    println!("five{:?} six{:?} none{:?}", five, six, none);


    // Placeholder for all other values
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    // what if you only care about a single value in a match? --> if let
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // can be re-written as `if let`
    // You do lose the exhaustive checking that `match` enforces
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
    
}
