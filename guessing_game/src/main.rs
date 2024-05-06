use std::io;
use rand::Rng;

fn get_number() {
    println!("Guess the number-- guess a number between 0 and 100!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let s: String = guess.trim().to_string();
    println!("{}", s);
    let my_int: i32 = match s.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not a valid number");
            return;
        }
    };

    if my_int < 0 || my_int > 100 {
        println!("Please input a number between 0 and 100.");
    }
    else {
        println!("You guessed: {guess}");
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(0..100));
    get_number();
}
