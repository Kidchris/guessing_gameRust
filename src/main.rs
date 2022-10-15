use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(0..101);
    println!("Can you guess the number i have in mind??");
    loop {
        println!("Enter your guess: ");
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("We didn't get you right!");
        println!("You guessed: {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("The value you entered is not a number");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is too low than what i predicted !"),
            Ordering::Equal => {
                println!("Your guess matched what i predicted !");
                break;
            }
            Ordering::Greater => println!("Your guess is greater than what i predicted !"),
        }
    }
}