#![allow(non_snake_case)]
/// Playing around with functions

// This is our main function. It is the first function that is called when the program is run.
fn main() {
    println!("This code is IN the main function");
    function_one(); // speaks for itself

    function_two(23); // Here, we pass the number 23 into function two.

    function_three(15, 'U'); // Characters MUST be in single quotes, remember?
    //TODO: finish
}

fn function_one() {
    println!("This code is NOT in the main function!")
}

/// This function allows us to give it a value, when calling it.
///
///         Types must be declared
///                 |
///                 V
fn function_two(x: i32) {
    println!("The value you gave function two is: {x}");
}

/// This function appends a character to a number.
fn function_three(first: i32, second: char) {
    println!("The combination is: {first}{second}");
}