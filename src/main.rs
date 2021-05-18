use std::io::{self, Write};
use std::iter::FromIterator;
fn main() {
    println!("Guess the phrase!");
    print!("Please input the phrase: ");
    io::stdout().flush().unwrap();
    let mut guessed: bool;
    let mut err = "".to_string();
    let mut secret = String::new();
    io::stdin()
        .read_line(&mut secret)
        .expect("Failed to read line");

    let secret_phrase = secret.trim();
    let mut guessed_chars = vec![' '];
    let mut guesses_left = 3;
    loop {
        match std::process::Command::new("clear").status() {
            Ok(_) => {}
            Err(_) => {
                // str.repeat wasn't working
                match std::process::Command::new("cls").status() {
                    Ok(_) => {}
                    Err(_) => {
                        println!("{}", "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
                    }
                }
            }
        }
        println!("{}", err);
        err = "".to_string();

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
        if guessed_chars.len() > 1 {
            println!(
                "Current guesses:{}",
                guessed_chars
                    .iter()
                    .map(|h| { h.to_string() })
                    .collect::<Vec<String>>()
                    .join(" ")
            );
        };
        print!("You have {} guess left, input your guess: ", guesses_left);
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let current_guess: char = match (guess.trim().chars().collect::<Vec<char>>()).first() {
            Some(e) => *e,
            None => {
                err = "You need to type a letter!".to_string();
                ' '
            }
        };
        if current_guess == ' ' {
            continue;
        };
        if guessed_chars.iter().any(|f| f == &current_guess) {
            err = "You already guessed this letter!".to_string();
            continue;
        }
        if secret_phrase
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .any(|f| f.to_ascii_lowercase() == current_guess.to_ascii_lowercase())
        {
        } else {
            err = format!("Nope, \"{}\" isn't there", current_guess);
            guesses_left -= 1
        };
        guessed_chars.sort();
        guessed_chars.push(current_guess);
    }
}
