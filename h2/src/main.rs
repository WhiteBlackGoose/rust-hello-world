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

enum MyResult<T, TError> {
    Ok(T),
    Error(TError)
}

fn main() {
    let myRes: MyResult<i32, &str> = MyResult::Ok(3);
    match myRes {
        MyResult::Ok(r) => println!("Ok: {r}"),
        MyResult::Error(e) => println!("Error: {e}")
    }
}
