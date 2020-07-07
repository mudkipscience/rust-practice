use std::io;

fn main() {
    println!("Number guessing game!");
    print!("Choose a number between 0 and 100: ")

    let mut guess = String::new();

    io.stdin()
        .read_line(&mut guess)
        
}
