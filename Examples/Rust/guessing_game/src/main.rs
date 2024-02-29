use std::{
    cmp::Ordering::{
        Equal, Greater, Less
    }, io, process::Termination
};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng()
                                .gen_range(1..=100);
    
    loop {    
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error: {}", e.to_string());
                println!("please enter number between 1 and 100");
                continue;
            },
        };
        
        match guess.cmp(&secret_number) {
            Less => println!("Too small!"),
            Greater => println!("Too big!"),
            Equal => {
                println!("You win!");
                break;
            },
        }
    }
}