// Enums are also easy
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// We can use it in structs
struct IpAddrStruct {
    kind: IpAddrKind,
    addr: String,
}

// We can do better
enum IpAddrString {
    V4(String),
    V6(String),
}

// We can do even better
enum IpAddrBetter {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Now blow your mind
struct Ipv4Addr {
    // whatever
}

struct Ipv6Addr {
    // whatever
}

// You can put any kind of data inside an enum variant
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

fn route(ip_kind: IpAddrKind) {
    println!("Ip kind {:?}", ip_kind);
}

// Instead of making lots of structs
// Then use Message as arguments in functions to win
#[derive(Debug)]
enum Message {
    Quit,                    // no data associated
    Move { x: i32, y: i32 }, // anonymous struct
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Something cool is that we can define methods on our enums
impl Message {
    fn call(&self) {
        // a method
    }
}

fn defining_an_enum() {
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let _home_struct = IpAddrStruct {
        kind: IpAddrKind::V4,
        addr: String::from("127.0.0.1"),
    };

    let _home_better = IpAddrString::V4(String::from("127.0.0.1"));

    let _home_even_better = IpAddrBetter::V4(127, 0, 0, 1);

    let msg = Message::Write(String::from("Hi there"));
    msg.call(); // so cool
    let msg = Message::Quit;
    msg.call();
    let msg = Message::Move { x: 1, y: 2 };
    println!("Move message {:?}", msg);
    msg.call();
    let msg = Message::ChangeColor(1, 2, 3);
    msg.call();

    // Even cooler is enum Option<t>
    let _some_num = Some(42);
    let _absent_num: Option<i32> = None;
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // skip
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    // Match is similar to a switch in C
    // It is also like an if, but the expression does not need to be a bool
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Match on the value
        Coin::Quarter(state) => {
            println!("State quarted from {:?}", state);
            25
        }
    }
}

fn maybe_inc(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(val) => Some(val + 1),
    }
}

fn match_flow() {
    value_in_cents(Coin::Quarter(UsState::Alabama));
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Nickel);

    // How about matching on Option<T>?
    let value_here = maybe_inc(Some(5));
    let no_value = maybe_inc(None);
}

fn concise_flow() {
    // If we want to match only one value, we need a lot of boilerplate
    let some8 = Some(8);
    match some8 {
        Some(3) => println!("three"),
        _ => (),
    }

    // We can do better with if let
    if let Some(3) = some8 {
        println!("three");
    }
    // We can add an else here
    else {
        // Handle all the other cases
        println!("Something else");
    }
}

fn main() {
    defining_an_enum();
    match_flow();
    concise_flow();
}
