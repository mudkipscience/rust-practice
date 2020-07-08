/* 
 * Simple number guessing game I made using the tutorial at 
 * https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html
 */

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // Generates random number between 0 and 100, thread_rng is the generator we're using
    let number = rand::thread_rng().gen_range(0, 101);
    let mut tries = 1;
    
    println!("Choose a number between 0 and 100: ");

    loop {
        // Defines a mutable variable that returns a string, don't know why constants exist yet
        let mut guess = String::new();

        // the & symbol in front of mut means its a reference, whatever that means lol
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // Remove whitespace and convert the input into an unsigned 32 bit integer, using match statement to handle err
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please type a number!");
                continue;
            }
        };

        // Checks whether the guess is higher, lower or equal to the random number
        // I added a tries thing myself, that tells you how many turns it took for you to guess the number
        match guess.cmp(&number) {
            Ordering::Less => {
                println!("Higher!");
                tries += 1;
            }
            Ordering::Greater => {
                println!("Lower!");
                tries += 1;
            }
            Ordering::Equal => {
                println!("You win! You took {} tries to guess the number.", tries);
                break;
            }
        }
    }
}
