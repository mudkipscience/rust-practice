/*
 * x is defined as an mutable variable, the value is reassigned again later without redfining it.
 * mutable variables are assigned one type, and the variable cannot be any other type other than the
 * one it was originally assigned.
*/

fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = 6;
    println!("The value of x is: {}", x);
}
