use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the Number");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess!");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Invalid number");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
}
