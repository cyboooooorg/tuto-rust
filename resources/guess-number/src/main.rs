use rand::prelude::*;
use std::io::Write;

fn get_user_input() -> Option<isize> {
    let mut input: String = String::new();

    if let Err(_) = std::io::stdin().read_line(&mut input) {
        println!("Error reading input");
        return None;
    }

    // Parse the input to a number
    match isize::from_str_radix(input.trim(), 10) {
        Ok(num) => Some(num),
        Err(_) => {
            println!("Invalid input");
            None
        }
    }
}

fn game() -> bool {
    let secret_number: isize = rand::thread_rng().gen_range(1..101);
    let max_attempts: i32 = 10;
    let mut attempts: i32 = 0;

    println!("The secret number is: {}", secret_number);

    loop {
        print!("Please input your guess: ");
        std::io::stdout().flush().unwrap();

        let guess = match get_user_input() {
            Some(num) => num,
            None => continue,
        };

        attempts += 1;

        if guess == secret_number {
            println!("You win!");
            break;
        } else if guess < secret_number {
            println!("Too small!");
        } else {
            println!("Too big!");
        }

        if attempts >= max_attempts {
            println!("You lose!");
            break;
        }
    }

    print!("Do you want to play again? (y/n): ");
    std::io::stdout().flush().unwrap();

    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input.trim().to_lowercase() == "y"
}

fn main() {
    println!("Guess the number!");

    loop {
        if !game() {
            break;
        }
    }
}
