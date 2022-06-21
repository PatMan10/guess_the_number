use std::io::stdin;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // let mut guess = String::new();
    // let mut guess = "".to_string();
    let mut guess = String::from("");
    let byte_size = stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);
    println!("Size: {}", byte_size);
}
