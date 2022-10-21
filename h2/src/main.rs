use rand::Rng;
use std::cmp::Ordering;

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

impl User {
    fn full_name(&self) -> String {
        let mut owned = self.username.clone();
        owned.push_str(&self.email);
        owned   
    }
    fn dummy(name: String) -> Self {
        User {
            active: false,
            username: name,
            email: String::new(),
            sign_in_count: 0
        }
    }
}

enum IpAddrKind {
    V4(String),
    V6(String),
    MyAss { x: i32, y: i32 }
}

fn main() {
    let user1 = User {
        email: String::from("heh@hehe.com"),
        username: String::from("goose"),
        active: true,
        sign_in_count: 1
    };
    println!("user1 is {:?}", user1.full_name());
    let four = IpAddrKind::V4(String::from("aaa"));
    let six = IpAddrKind::V6(String::from("bbb"));
    let x = match IpAddrKind::MyAss { x: 32, y: 44 } {
        IpAddrKind::V4(&s) => String::from("v4 {s}"),
        IpAddrKind::V6(&s) => String::from("v6 {s}"),
        IpAddrKind::MyAss{ x: x, y: y} => String::from("my ass {x} {y} ")
    };
}
