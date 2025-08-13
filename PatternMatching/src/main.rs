
enum IpAddrKind {
    V4(String),
    V6(String)
}


enum Message {
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn some() {
        println!("Recebaaaa!!!");
    }
}

/*struct IpAddr {
    kind: IpAddrKind,
    adress: String
}*/

fn main() {
    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from("127.0.0.1"));

    /*let localhost = IpAddr {
        kind: IpAddrKind::V4,
        adress: String::from("127.0.0.1")
    }; É um jeito de se declarar uma coisa com tipo de enum*/

    Message::some();
    route(four);

    let a = 8;
    let b = Some(2);

    let sum = a + b.unwrap_or(0);
    println!("{} + {} = {}", a, b.unwrap_or(0), sum);

    if let Err(e) = its_useful(Coin::Penny) {
        println!("Error: {e}");
    }

}

fn route(ip: IpAddrKind) {

    match ip {
        IpAddrKind::V4(text) => println!("IPV4: {}", text),
        IpAddrKind::V6(text) => println!("IPV6: {}", text)
    }

    value_in_cents(Coin::Quarter(State::Sp));

    

}

#[derive(Debug)]
enum State {
    Sp,
    Mg,
    Rj,
    Ba
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(st) => {
            println!("This quarter is from {:?}", st);
            1
        },
    }
}

fn its_useful(coin: Coin) -> Result<(), String> {
    match coin {
        Coin::Dime => Ok(()),
        Coin::Quarter(s) => Ok(()),
        _ => Err("Not useful".to_string())
    }
}
