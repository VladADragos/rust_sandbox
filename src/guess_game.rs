#![allow(dead_code)]

mod guess_game {

    use std::io;
    extern crate rand;

    pub fn guess_game() {
        let secret_num: u8 = rand::random::<u8>() % 10;
        let max_guess: u8 = 3;
        greet();

        for i in 0..max_guess {
            let guess = get_guess(i, max_guess);
            if guess == secret_num {
                println!("you won!");
                return;
                // break;
            }
        }
        println!("Sorry, you lost. My secret number was {}", secret_num);
    }

    fn get_guess(guess_count: u8, max_guesses: u8) -> u8 {
        loop {
            println!("{}/{}", guess_count + 1, max_guesses);
            println!("Guess: ");
            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("could not read stdin");
            match guess.trim().parse::<u8>() {
                Ok(v) => return v,
                Err(e) => println!("invalid input, {}", e),
            }
        }
    }

    fn greet() {
        println!("Welcome to my number guessing name, can you guess my secret number? guess my secret number, its between 0 and 10.");
    }
}
