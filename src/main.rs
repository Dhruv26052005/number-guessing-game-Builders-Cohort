use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("=== Guess the Number ===");
    println!("I'm thinking of a number between 1 and 100.");
    println!("You have 5 attempts to guess it.\n");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut attempts = 0;
    let max_attempts = 5;

    loop {
        let remaining = max_attempts - attempts;
        println!("Enter your guess ({} left):", remaining);

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That doesn't look like a number. Try again!\n");
                continue;
            }
        };

        if guess < 1 || guess > 100 {
            println!("Please enter a number between 1 and 100.\n");
            continue;
        }

        attempts += 1;
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small! Try a bigger number.");
                print_hint(secret_number, guess);
                println!();
            }
            Ordering::Greater => {
                println!("Too big! Try a smaller number.");
                print_hint(secret_number, guess);
                println!();
            }
            Ordering::Equal => {
                println!("\nYou got it! The number was {secret_number}.");
                println!("It took you {} attempt(s). Nice work!\n", attempts);
                break;
            }
        }

        if attempts >= max_attempts {
            println!("\nNo more attempts. The number was {secret_number}.\n");
            break;
        }
    }
}

fn print_hint(secret_number: u32, _guess: u32) {
    let lower = secret_number.saturating_sub(8).max(1);
    let upper = (secret_number + 8).min(100);
    println!("Hint: {lower} < x < {upper}");
}
