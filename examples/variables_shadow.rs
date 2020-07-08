/*
 * x is defined as an immutable variable, then defined again later (shadowed?)
 * redefining a variable like this lets you change the type.
*/

fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = 6;
    println!("The value of x is: {}", x);
}
