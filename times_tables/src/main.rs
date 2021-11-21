use chrono::Utc;
use rand::Rng;
use std::io::{self, Write};

const TITLE: &str = "
 _____ _                       _____      _     _
/__   (_)_ __ ___   ___  ___  /__   \\__ _| |__ | | ___  ___
  / /\\/ | '_ ` _ \\ / _ \\/ __|   / /\\/ _` | '_ \\| |/ _ \\/ __|
 / /  | | | | | | |  __/\\__ \\  / / | (_| | |_) | |  __/\\__ \\
 \\/   |_|_| |_| |_|\\___||___/  \\/   \\__,_|_.__/|_|\\___||___/
";

fn main() {
    println!("{}", TITLE);
    let mut score = 0;
    let mut lives = 3;
    let seconds = 5;
    println!("You have {} lives. Lets see how many rounds you can complete.\nYou have {} seconds to answer each question.", lives, seconds);

    loop {
        // Check if we are out of lives.
        if lives < 1 {
            println!("GAME OVER! You scored {}", score);
            break;
        }

        let left = rand::thread_rng().gen_range(1, 12);
        let right = rand::thread_rng().gen_range(1, 12);
        let answer = left * right;

        let start_time = Utc::now().time();

        print!("{} * {} = ", left, right);
        io::stdout().flush().unwrap();
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed.");
        let end_time = Utc::now().time();
        guess = guess.trim().to_string();

        // Check if the user took too long to answer.
        let diff = end_time - start_time;
        if diff.num_seconds() > seconds {
            println!(
                "Uh oh you took {} seconds, you only have {} to answer.",
                diff.num_seconds(),
                seconds
            );
            lives = lives - 1;
            continue;
        }

        // Check if their guess was correct
        if !answer.to_string().eq(&guess) {
            println!(
                "Incorrect!  The correct answer is {}, you have {} lives remaining.",
                answer, lives
            );
            lives = lives - 1;
            continue;
        }

        println!("Correct! You have scored {} so far.", score);
        score = score + 1;
    }
}
