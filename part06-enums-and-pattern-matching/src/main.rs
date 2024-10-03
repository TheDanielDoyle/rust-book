use message::Message::*;

// 6. Enums and Pattern Matching
fn main() {

    // my_mod::function();
    // message::Message
    // message::Message::*;

    // 6.1 Defining an Enum
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    fn route(ip_type: IpAddrKind) {
        println!("{:?}", ip_type)
    }

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String)
    }

    let home1 = IpAddr::V6(String::from("127.0.0.1"));
    // let loopback = IpAddr::V6(String::from("::1"));
    let home2 = IpAddr::V4(127, 0, 0, 1);
    if let IpAddr::V4(p1, p2, p3, p4) = home2 {
       println!("{}, {}, {}, {}", p1, p2, p3 ,p4)
    }

    // struct Ipv4Addr {
    //     // --snip--
    // }

    // struct Ipv6Addr {
    //     // --snip--
    // }

    // enum IpAddr {
    //     V4(Ipv4Addr),
    //     V6(Ipv6Addr),
    // }

    // enum Message {
    //     Quit,
    //     Move { x: i32, y: i32 },
    //     Write(String),
    //     ChangeColor(i32, i32, i32),
    // }

    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct

    // impl Message {
    //     fn call(&self) {
    //         // method body would be defined here
    //     }
    // }
    //
    // let m = Message::Write(String::from("hello"));
    // m.call();

    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    let some_number = Some(4);
    let some_char = Some('a');

    let absent_number: Option<i32> = None; // core::option::Option::None

    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    //
    // let sum = x + y;

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    let c = Coin::Penny;
    let amount = match c {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    };

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
    let _ = value_in_cents(Coin::Nickel);

    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }
    let state = UsState::Alabama;


    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // enum Message {
    //     Quit,
    //     Move { x: i32, y: i32 },
    //     Write(String),
    //     ChangeColor(i32, i32, i32),
    // }

    let message = ChangeColor(1,2,3);

    // use Message::Quit;

    let y = match message {
        Quit => {},
        Move { x, y } => {
            println!("{}", x);
            println!("{}", y);
        },
        Write(_) => {},
        ChangeColor(_, _, _) => {},
    };

}