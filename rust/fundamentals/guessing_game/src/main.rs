use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    loop{

           let secret_number = rand::thread_rng().gen_range(1..100);
            println!("Please input your guess.");

            let mut guess = String::new();    
            

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
            };

            
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => println!("You win!"),
            }

            println!("You guessed: {}", guess);
            println!("The secret number was: {}", secret_number);

            if guess == secret_number {
                println!("Congratulations! You've guessed the number!");
                break;
            } else {
                println!("Try again!");
            }

    }

 
}
