use std::io;

fn main() {
    println!("This program can convert between celcius and farenheight.");
    println!("Which unit would you like to convert to?");

    let mut unit = String::new();
    let mut temp = String::new();

    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read line.");

    unit = unit.trim().to_lowercase();

    println!("Please type in the temperature:");

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line.");

    let temp_trimmed = temp.trim();
    let temp_f64: f64 = temp_trimmed.parse()
        .expect("Invalid input, please enter an valid integer!");

    println!("{}", temp_f64);

    if unit == "fahrenheit" || unit == "f" {
        let output = to_f(temp_f64);
        println!("{}C is equal to {}F", temp_f64, output);
    } else if unit == "celcius" || unit == "c" {
        let output = to_c(temp_f64);
        println!("{}F is equal to {}C", temp_f64, output);
    } else {
        println!("Input is not a valid temperature unit, please try again.");
    }
}

fn to_f(input: f64) -> f64 {
    (input * 9.0/5.0) + 32.0
}

fn to_c(input: f64) -> f64 {
    (input - 32.0) * 5.0/9.0
}
