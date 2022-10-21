use rand::Rng;
use std::cmp::Ordering;


fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("Guess the number");
    let _sec: i32 = rand::thread_rng().gen_range(1..=1000000);
    let b = {
        let a = [1, 2, 3];
        sum(a[0], a[1])
    };
    let result = {
        let mut i = 0;
        loop {
            i += 1;
            if i > 5 { break i; }
        }
    };
    println!("{result}");

    for el in (1..5).rev() {
        println!("{el}");
    }
}
