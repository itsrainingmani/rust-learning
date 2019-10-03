#![allow(unused_variables)]
fn main() {
    println!("Chapter 6 - Enums and Pattern Matching");

    struct Ipv4Addr {}

    struct Ipv6Addr {}

    // You can put any kind of data inside Enums (string, numerics, strucs, enums etc)
    enum IpAddr {
        // V4(String),
        V4(u8, u8, u8, u8),
        // V4(Ipv4Addr),
        V6(String),
        // V6(Ipv6Addr),
    }
    // let home = IpAddr::V4(String::from("127.0.0.1"));
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    enum Message {
        Quit,                       // No data
        Move { x: i32, y: i32 },    // Anonymous struct
        Write(String),              // Single string
        ChangeColor(i32, i32, i32), // Three i32 vals
    }

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
        Dollar,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky Penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State Quarter from {:#?}", state);
                match state {
                    UsState::Alabama => println!("{:#?}'s motto: Roll Tide", state),
                    UsState::Alaska => println!("{:#?}'s motto: Oil? idk", state),
                }
                25
            }
            Coin::Dollar => 100,
        }
    }

    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Quarter(UsState::Alaska));

    fn plus_one(x: Option<i32>) -> Option<i32> {
        // Match is exhaustive. Gives compiler error if all possible values are not handled
        match x {
            Some(i) => Some(i + 1),
            None => None,
        }
    }

    // In order to have a value that can possibly be null,
    // You must explicitly opt in by making the type of that value, Option<T>
    // This requires you to explicitly handle the case when the value is null
    // A value that has a type that isn't an Option<T> can be safely assumed
    // to not be null

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_val = Some(0u8);
    match some_u8_val {
        Some(3) => println!("three"),
        _ => (), // _ will match on all cases not specified. () is the unit return
    }

    // The above code is a little verbose for only handling Some(3)
    // Easier to use if let
    if let Some(3) = some_u8_val {
        println!("three");
    }

    // if let is syntax suger for a match that runs code when the value matches
    // one pattern and ignores all other values
}
