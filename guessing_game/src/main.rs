use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

/// This is a simple guessing game.
fn main() {
    println!("Welcome to the guessing game.");

    println!("What is your name?");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name.");

    name = name.trim().to_string();
    println!("Hello {}!", name);

    loop {
        let mut tries = 0;

        println!("");

        let secret_number = rand::thread_rng().gen_range(1, 101);
        println!("I have picked a number between 1 and 100.\nCan you guess what it is?");

        loop {
            print!("Type your guess and press [enter]:");
            io::stdout().flush().unwrap();

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            if guess.eq(&"quit".to_string()) {
                break;
            }

            // the : used here is to provide annotation
            // information to the rest of the instructions.
            // parse() can now infer that it should use u32
            // as the target type when parsing the string guess value.
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("You have to guess a number between 1 and 100.");
                    continue;
                }
            };

            tries = tries + 1;

            match guess.cmp(&secret_number) {
                Ordering::Less => {
                    println!("");
                    println!("{} is too low! Try again.", guess);
                    println!("");
                }
                Ordering::Greater => {
                    println!("");
                    println!("{} is too high! Try again.", guess);
                    println!("");
                }
                Ordering::Equal => {
                    println!("");
                    println!(
                        "Yes it was {}, Well done {}! It took you {} guesses!",
                        secret_number, name, tries
                    );
                    println!("Press [enter] play again.");
                    let mut temp = String::new();
                    io::stdin()
                        .read_line(&mut temp)
                        .expect("Failed to read line");

                    break;
                }
            }
        }
    }
}
