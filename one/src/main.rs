use rand::{Rng};
use std::io;
use std::cmp::Ordering;

fn main () {
    let n = rand::thread_rng().gen_range(1..101);
    println!("{}", n);
    loop{
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue
            }
        };
        match guess.cmp(&n) {
            Ordering::Less => println!("small"),
            Ordering::Greater => println!("big"),
            Ordering::Equal => {
                println!("win!");
                break;
            }
        }
    }
}