// This file contains the implementation of a simple game called "Rock Paper Scissors".

use std::io;

fn main() {
    println!("Welcome to Rock Paper Scissors!");

    let user_choice = get_user_input();

    if user_choice == "rock" || user_choice == "paper" || user_choice == "scissors" {
        println!("{}", user_choice);
        match user_choice {
            "rock" => println!("{}", "You chose rock! I chose {choice}.",),
            "paper" => println!("{}", "You chose paper! I chose {choice}.",),
            "scissors" => println!("{}", "You chose scissors! I chose {choice}.",),
            _ => unreachable!("User's choice is not valid."),
        }
    } else {
        println!("Invalid input. Please try again.");
    }

    // Get the user's first move
    let user_input = get_user_input();

    if user_input == "rock" || user_input == "paper" || user_input == "scissors" {
        match user_input {
            "rock" => println!("{}", "You won! I chose {choice}.",),
            "paper" => println!("{}", "You lost. I chose {choice}.",),
            "scissors" => println!("{}", "You are the winner! {choice} beats me!",),
            _ => unreachable!("User's input is not valid."),
        }
    } else {
        println!("Invalid move. Please try again.");
    }
}

/// Get a user's choice from stdin
fn get_user_input() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    buffer.trim().to_string()
}
