// 6. Enums and Pattern Matching
fn main() {
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
    let home2 = IpAddr::V4(127, 0, 0, 1);
    if let IpAddr::V4(p1, p2, p3, p4) = home2 {
       println!("{}, {}, {}, {}", p1, p2, p3 ,p4)
    }
}
