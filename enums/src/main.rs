#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Colorado
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {

    let loopback = IpAddr::V6(String::from("::1"));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => {
            println!("State quarter from {:?}!", state);    
            25 
        }
    }
}
