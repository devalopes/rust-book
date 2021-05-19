use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number between 1 and 100!\n");

    // thread_rng gives us the particular random number generator we're going to use
    // one that is local to current execution thread & seeded by the OS
    // gen_range is inclusive..exclusive (e.g. 1..101)
    // gen_range can also be written like inclusive..=inclusive (e.g. 1..=100)
    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("Enter a valid number between 1 and 100");continue}
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{} is too low!", guess),
            Ordering::Greater => println!("{} is too high!", guess),
            Ordering::Equal =>  {println!("You win!!"); break}
        }
    }


}