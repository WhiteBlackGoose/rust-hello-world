use rand::Rng;
use std::cmp::Ordering;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct Color(i32, i32, i32);

fn first_word(s: &str) -> &str {
    let mut i: usize = 0;
    for (i, &c) in s.as_bytes().iter().enumerate() {
        if c == b' ' {
            return &s[..i];
        } 
    }
    &s[..]
}

fn main() {
    let s: String = String::from("Hello world aaa");
    let w = first_word(&s);
    let ggg = &s[0..1];
    println!("{w}");
    let user1 = User {
        email: String::from("aaa"),
        username: String::from("aaa"),
        active: true,
        sign_in_count: 1
    };
    user1.email = String::from("aaaa");
}
