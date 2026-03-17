//! A program for a number guessing game - The player needs to guess a number between 1-100.

use rand::Rng;
use std::{cmp::Ordering, fmt::Debug, io, str::FromStr};

/// secret number ranges.
const MIN_NUMBER_RANGE: i32 = 1;
const MAX_NUMBER_RANGE: i32 = 100;

fn main() {
    println!("Welcome to guess the number game!");
    let secret_number = rand::thread_rng().gen_range(MIN_NUMBER_RANGE..=MAX_NUMBER_RANGE);
    guessing_game(secret_number);
}

/// The guessing number game - Receives a secret number and lets the player guess the number until right while giving clues based on the guesses.
///
/// # Parameters
/// `secret_number`: The number the player needs to guess.
fn guessing_game(secret_number: i32) {
    loop {
        println!(
            "Please input your guess ({}-{})",
            MIN_NUMBER_RANGE, MAX_NUMBER_RANGE
        );
        let guess: i32 = read_input("Invalid number");

        if !(MIN_NUMBER_RANGE..=MAX_NUMBER_RANGE).contains(&guess) {
            println!(
                "The guess is not in the range {}-{}\n",
                MIN_NUMBER_RANGE, MAX_NUMBER_RANGE
            );
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

/// Receives and returns the user's input.
///
/// # Returns
/// The user's input as String.
///
/// # Panics
/// If `read_line()` fails panic with `Failed to read line` message.
fn receive_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

/// Receives input from the user and parse it to the wanted type.
///
/// # Parameters
/// `error`: The error to raise in case the function fails.
///
/// # Returns
/// The received user's input in the wanted type.
///
/// # Panics
/// If `receive_input().trim().parse()` fails panics with the received error.
///
/// # Examples
/// `let num: i32 = read_input("Invalid number");`
fn read_input<T: FromStr>(error: &str) -> T
where
    T::Err: Debug,
{
    receive_input().trim().parse().expect(error)
}
