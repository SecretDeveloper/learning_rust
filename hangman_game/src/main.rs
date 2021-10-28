use std::io::{self, Write};

const TITLE: &str = "
 __ __   ____  ____    ____  ___ ___   ____  ____
|  |  | /    ||    \\  /    ||   |   | /    ||    \\
|  |  ||  o  ||  _  ||   __|| _   _ ||  o  ||  _  |
|  _  ||     ||  |  ||  |  ||  \\_/  ||     ||  |  |
|  |  ||  _  ||  |  ||  |_ ||   |   ||  _  ||  |  |
|  |  ||  |  ||  |  ||     ||   |   ||  |  ||  |  |
|__|__||__|__||__|__||___,_||___|___||__|__||__|__|
";

const YOU_WIN: &str = "
 __ __   ___   __ __      __    __  ____  ____
|  |  | /   \\ |  |  |    |  |__|  ||    ||    \\
|  |  ||     ||  |  |    |  |  |  | |  | |  _  |
|  ~  ||  O  ||  |  |    |  |  |  | |  | |  |  |
|___, ||     ||  :  |    |  `  '  | |  | |  |  |
|     ||     ||     |     \\      /  |  | |  |  |
|____/  \\___/  \\__,_|      \\_/\\_/  |____||__|__|

";

const GAME_OVER: &str = "
  ____   ____  ___ ___   ___        ___   __ __    ___  ____
 /    | /    ||   |   | /  _]      /   \\ |  |  |  /  _]|    \\
|   __||  o  || _   _ |/  [_      |     ||  |  | /  [_ |  D  )
|  |  ||     ||  \\_/  ||    _]    |  O  ||  |  ||    _]|    /
|  |_ ||  _  ||   |   ||   [_     |     ||  :  ||   [_ |    \\
|     ||  |  ||   |   ||     |    |     | \\   / |     ||  .  \\
|___,_||__|__||___|___||_____|     \\___/   \\_/  |_____||__|\\_|

";

const HANGMAN_ICONS: [&str; 7] = [
    "
   +---+
   |   |
       |
       |
       |
       |
   =========",
    "
   +---+
   |   |
   O   |
       |
       |
       |
   =========",
    "
   +---+
   |   |
   O   |
   |   |
       |
       |
   =========",
    "
   +---+
   |   |
   O   |
  /|   |
       |
       |
   =========",
    "
   +---+
   |   |
   O   |
  /|\\  |
       |
       |
   =========",
    "
   +---+
   |   |
   O   |
  /|\\  |
  /    |
       |
   =========",
    "
   +---+
   |   |
   O   |
  /|\\  |
  / \\  |
       |
   =========",
];

fn main() {
    loop {
        // Clear scren
        print!("\x1B[2J");
        io::stdout().flush().unwrap();

        // Show HANGMAN title
        println!("{}", TITLE);

        // Capture secret word
        print!("Enter the secret word and press [enter]:");
        io::stdout().flush().unwrap();
        let mut secret_word = String::new();
        io::stdin().read_line(&mut secret_word).expect("Failed.");
        secret_word = secret_word.trim().to_ascii_uppercase(); // only use uppercase

        let mut masked_word = String::new(); // showing _ and correct guesses
        let mut guessed_letters = String::new(); // storing guessed letters

        let mut guess_count = 0;
        loop {
            // clear screen
            print!("\x1B[2J");
            io::stdout().flush().unwrap();

            // Show HANGMAN title
            println!("{}", TITLE);

            // show gallows
            println!("{}", HANGMAN_ICONS[guess_count]);
            // check if we are out of guesses
            if guess_count > 5 {
                println!("");
                println!("{}", GAME_OVER);
                println!("");
                println!("Press [enter] to play again or CTRL+C to quit.");
                io::stdin().read_line(&mut masked_word).expect("Failed.");
                break;
            }

            // Show masked words
            masked_word = "".to_string(); // clear the mask

            // recreate the mask.
            for c in secret_word.chars() {
                if guessed_letters.contains(c) {
                    masked_word.push(c);
                } else {
                    masked_word.push('_');
                }
            }

            // check if we have already won
            if masked_word.eq(&secret_word) {
                println!("");
                println!("{}", YOU_WIN);
                println!("");
                println!("Press [enter] to play again or CTRL+C to quit.");
                io::stdin().read_line(&mut masked_word).expect("Failed.");
                break;
            }

            // PRINT the mask
            println!("");
            println!("\t\t{}", masked_word);
            println!("");

            // get guessed letter and add it to guessed_letters string.
            let mut guess = String::new();
            print!("Type a letter and press [enter]:");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut guess).expect("Failed.");
            // convert to upper case.
            guess = guess.to_ascii_uppercase();

            // some special handling
            // if it starts with a SPACE then trim it to just a space.
            if guess.chars().next().unwrap() == ' ' {
                guess = " ".to_string(); // essentially trimming the \n
            } else if guess.trim().chars().count() == 0 {
                // if it doesnt start with SPACE, and is only trimmable chars
                // then loop around assuming invalid key hit.
                continue; // skip loop around if we did not get a value.
            }
            // get the first character entered
            let ch = guess.chars().next().unwrap();
            guessed_letters.push(ch);

            // If guess is not contained in secret_word then increment guess count
            if secret_word.contains(ch) == false {
                guess_count = guess_count + 1;
            }
        }
    }
}
