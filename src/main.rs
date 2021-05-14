use std::io::{self, Write};
use std::iter::FromIterator;
use std::{thread, time};

fn main() {
    println!("Guess the phrase!");
    let dur = time::Duration::from_millis(1500);
    print!("Please input the phrase: ");
    io::stdout().flush().unwrap();
    let mut guessed: bool;
    let mut secret = String::new();
    io::stdin()
        .read_line(&mut secret)
        .expect("Failed to read line");

    let secret_phrase = secret.trim();
    let mut guessed_chars: Vec<char> = vec![' '];
    let mut guesses_left = 3;
    loop {
        match std::process::Command::new("clear").status() {
            Ok(_) => {}
            Err(_) => {
                println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
            }
        }

        if guesses_left == 0 {
            println!("You lost! The phrase was: '{}'", secret_phrase);
            break;
        }
        guessed = true;
        println!(
            "{}",
            String::from_iter(
                secret_phrase
                    .chars()
                    .collect::<Vec<char>>()
                    .iter()
                    .map(|x| {
                        if guessed_chars
                            .iter()
                            .any(|i| i.to_ascii_lowercase() == x.to_ascii_lowercase())
                        {
                            x
                        } else {
                            {
                                guessed = false;
                                &'_'
                            }
                        }
                    })
            )
        );
        if guessed {
            println!("You won!");
            break;
        }
        print!("You have {} guesses left, input your guess: ", guesses_left);
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let current_guess: char = match (guess.trim().chars().collect::<Vec<char>>()).first() {
            Some(e) => *e,
            None => {
                println!("You need to type a letter!");
                thread::sleep(dur);
                ' '
            }
        };
        if current_guess == ' ' {
            continue;
        };
        if guessed_chars.iter().any(|f| f == &current_guess) {
            println!("You already guessed this letter!");
            thread::sleep(dur);
            continue;
        }
        if secret_phrase
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .any(|f| f.to_ascii_lowercase() == current_guess.to_ascii_lowercase())
        {
        } else {
            println!("Nope!");
            thread::sleep(dur);
            guesses_left -= 1
        };
        guessed_chars.push(current_guess);
    }
}
