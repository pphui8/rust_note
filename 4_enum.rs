enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    v4(u8, u8, u8, u8),
    v6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    // as this a function
    Write(String),
    ChangeColor(i32, i32, i32),
}

// same as struct
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let ipv4 = IpAddr::v4(127, 0, 0, 1);
    let ipv6 = IpAddr::v6(String::from("127.0.0.1"));
}

fn route(ip_kind: IpAddrKind) {}