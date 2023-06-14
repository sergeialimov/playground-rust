
use std::io::stdin;
use rand::Rng;
use std:: cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    println!("secret number is {secret_number}");

    println!("Please input your guess.");

    let mut guess: String = String::new();

    stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("please type a number");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
