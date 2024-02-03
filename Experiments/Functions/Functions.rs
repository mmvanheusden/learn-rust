#![allow(non_snake_case)]

/// Playing around with functions, and explaining statements and expressions
// This is our main function. It is the first function that is called when the program is run.
fn main() {
    println!("This code is IN the main function");
    function_one(); // speaks for itself

    function_two(23); // Here, we pass the number 23 into function two.

    function_three(15, 'U'); // Characters MUST be in single quotes, remember?

    let y = 6; // <-- This is a statement. It doesn't return a value. It just does it.

    //let x = (let y = 6) <-- This won't work, because the statement doesn't return anything we also can't put it in a variable.

    let x = {
        let x = 3; // <-- This is a statement
        /*

      This is an expression. It returns x + 1.
          |
          |
          V                                        */
        x + 1 // this returns x + 1, so 4.
        // Coud've been 'return x+1'. This is just shorter.
    };

    println!("x: {x}");

    /*
    STATEMENTS AND EXPRESSIONS

    * A STATEMENT is an instruction that does not return a value.
    * AN EXPRESSION evaluates to a resulting value.
     */




    let a = thirteen();
    println!("a is {a}");


    let b = minus_one(a);
    println!("a minus one is {b}");
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

/// This function has a return value. It returns a value
/// We specify the type of the value it returns.
fn thirteen() -> i32 {
    // return 13
    13
}


/// This returns a value as well.
/// -1 of it's input
fn minus_one(x: i32) -> i32 {
    // return x - 1
    x - 1 // <-- Must not have a semicolon, because it's an expression
}