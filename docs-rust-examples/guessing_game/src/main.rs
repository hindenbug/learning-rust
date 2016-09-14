extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

// Rust's built in number types i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64

fn ask_for_input() -> u32{
    println!("Please input your guess!");

    let mut guess = String::new();
    let reader = io::stdin();

    // expect() used in case read_line() crashes
    reader.read_line(&mut guess).ok().expect("Failed to read line");

    // shadow existing var name with new;
    // Shadowing lets us re-use the var name

    // .parse() method on strings parses into some other type of number. Ex: into u32
    //  or '(turbofish)' ::<> [ ex: "somestring".parse::<u32>(); ]
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    return guess
}

fn generate_secret_number() -> u32{
    return rand::thread_rng().gen_range(1, 101);
}


fn main() {
    println!("Guessing game !");

    let secret_number = generate_secret_number();
    let guess = ask_for_input();

    println!("The secret number is {}", secret_number);
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You Win!"),
    }

}
