use rand::thread_rng;
use rand::Rng;
use std::io::stdin;

fn main() {
    println!("-------- GUESS THE NUMBER --------\n");
    println!("The computer is thinking of a number...");
    println!("It's between 1 and 10.");
    println!("Can you guess what it is?");

    let secret_number = thread_rng().gen_range(1..11);
    let mut guess = String::new();
    stdin().read_line(&mut guess).expect("Failed to read line");

    println!("\nYou guessed {}", guess);
    println!("The secret number is {}", secret_number);
}
