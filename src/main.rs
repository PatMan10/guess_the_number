use rand::thread_rng;
use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!("-------- GUESS THE NUMBER --------\n");
    println!("The computer is thinking of a number...");
    println!("It's between 1 and 10.");
    println!("Can you guess what it is?");

    let rand_number = thread_rng().gen_range(1..11);

    loop {
        println!("Enter your guess: ");

        let mut guess = String::new();

        stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        match guess.cmp(&rand_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("You guessed it!");
                println!("The secret number is {}", rand_number);
                break;
            }
        }
    }
}
