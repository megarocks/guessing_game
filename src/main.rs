extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1,101);
    let mut suggestions = 0;

    loop {
        println!("Please guess a number =>");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Can't read a string");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your suggestion: {}", guess);
        suggestions = suggestions + 1;

        match guess.cmp(&secret_number) {
            Ordering::Less      =>  println!("Too Small"),
            Ordering::Greater   =>  println!("Too big"),
            Ordering::Equal     =>  {
                println!("You Won after {} suggestions", suggestions);
                break;
            }
        }
    }

}
