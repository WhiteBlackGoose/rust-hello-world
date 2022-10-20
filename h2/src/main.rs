use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");
    let sec = rand::thread_rng().gen_range(1..=1000000);
    loop {
        println!("Please input your guess");
    
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&sec) {
            Ordering::Less => println!("Too little"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You won");
                break;
            },
        }
    }
}
