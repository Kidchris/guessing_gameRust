use std::io;

fn main() {
    println!("Can you guess the number i have in mind??");
    println!("If so? Enter your guess: ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("We didn't get you right!");
    println!("You guessed: {}", guess);
}
