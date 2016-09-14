extern crate rand;

use std::io;
use rand::Rng;

fn ask_for_input() -> String{
    println!("Please input your guess!");

    let mut guess = String::new();
    let reader = io::stdin();

    reader.read_line(&mut guess).ok().expect("Failed to read line");

    return guess;
}

fn generate_secret_number() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is {}", secret_number);
}


fn main() {
    println!("Guessing game !");

    generate_secret_number();
    let guess = ask_for_input();

    println!("You guessed: {}", guess);

}
