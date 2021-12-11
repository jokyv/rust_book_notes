#[derive(Debug)]
// declare enum UsState
enum UsState {
    Alabama,
    Alaska,
    Florida,
}

// declate enum coin
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// function for matching coins
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            // return 1
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            // return 25
            25
        }
    }
}

// OPTIONS MATCH
fn plus_one(x: Option<i32>) -> Option<i32> {
    // return either some(i) or None
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn print_only_tens(x: Option<u8>) {
    match x {
        Some(10) => println!("each 10!"),
        _ => (), // will print nothing, new line
    }
}

// main function
fn main() {
    // construct a coin
    let penny = Coin::Penny;
    // returns 1 from the function value_in_cents
    let result1 = value_in_cents(penny);
    println!("the result from coin is {}", result1);
    println!("");

    // construct a state
    let florida = Coin::Quarter(UsState::Florida);
    // returns 25 as florida is a Quarter coin
    let result2 = value_in_cents(florida);
    println!("the result from state is {}", result2);
    println!("");

    // prints 5 or None
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six is {:?}, none is {:?}", six, none);
    println!("");

    // showcase how to use _
    print_only_tens(Some(1));
    print_only_tens(Some(10));
    print_only_tens(Some(3));
}
