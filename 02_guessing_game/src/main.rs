use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {

    let secret_nr = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Guess the number!");
        print!("> ");


        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed reading line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_nr) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}