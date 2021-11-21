use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

const TITLE: &str = "
 ___ ___   ____  ______  __ __  _____      ___   __ __  ____  _____
|   |   | /    ||      ||  |  |/ ___/     /   \\ |  |  ||    ||     |
| _   _ ||  o  ||      ||  |  (   \\_     |     ||  |  | |  | |__/  |
|  \\_/  ||     ||_|  |_||  _  |\\__  |    |  Q  ||  |  | |  | |   __|
|   |   ||  _  |  |  |  |  |  |/  \\ |    |     ||  :  | |  | |  /  |
|   |   ||  |  |  |  |  |  |  |\\    |    |     ||     | |  | |     |
|___|___||__|__|  |__|  |__|__| \\___|     \\__,_| \\__,_||____||_____|

";

fn main() {
    println!("{}", TITLE);

    const MAX_RANGE: i32 = 12;
    loop {
        let num1 = rand::thread_rng().gen_range(1, MAX_RANGE);
        let num2 = rand::thread_rng().gen_range(1, MAX_RANGE);
        let answer = num1 + num2;

        loop {
            println!("");
            println!("What is {} + {} ?", num1, num2);

            let mut guess = String::new();
            io::stdin().read_line(&mut guess).expect("Failed.");
            guess = guess.trim().to_string();

            if answer.to_string().eq(&guess) {
                println!("Correct!");
                break;
            } else {
                println!("Incorrect, try again.");
            }
        }
    }
}
