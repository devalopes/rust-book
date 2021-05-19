use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // thread_rng gives us the particular random number generator we're going to use
    // one that is local to current execution thread & seeded by the OS
    // gen_range is inclusive..exclusive (e.g. 1..101)
    // gen_range can also be written like inclusive..=inclusive (e.g. 1..=100)
    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed {}", guess);
}