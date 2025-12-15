#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[allow(dead_code)]
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[allow(dead_code)]
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let _four = IpAddrKind::V4; // namespaced under the identifier IpAddrKind
    let _six = IpAddrKind::V6;

    route(&_four);
    println!("{_four:?}");

    let _ip4 = IpAddr::V4(String::from("127.0.0.1"));
    let _ip6 = IpAddr::V6(String::from("::1"));

    route_ip_addr(&_ip6);

    let m = Message::Write("Hello World".to_string());
    m.call();
}

fn route(ip_kind: &IpAddrKind) {
    // will print "[src/main.rs:16:5] &ip_kind = V4"
    dbg!(ip_kind);
}

fn route_ip_addr(ip: &IpAddr) {
    dbg!(ip);
}
