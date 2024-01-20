use rand::Rng;
use std::{cmp::Ordering, io}; // We import the standard input-output library. Without it, we can't interact with the terminal.

fn main() {
    /*                                      If we woudn't have added this =, it would not go further than 99
                                                            |
                                                            |
    //                                                      V   */
    let random_number = rand::thread_rng().gen_range(1..=100); // We create a variable and fill it with a random number between 1 and 100.

    // println!("{random_number}"); debug

    loop {
        println!("Guess the number! Please give me your guess: (must be between 1 and 100)");
        /* the String::new() in the guess var creation, means that it is assigned a String type.
           This String type is growable, and encoded in UTF-8.
        The ::new part creates a new String. It's an associated function of something, in this case String. */

        // This variable is mutable. that means we can't change it.
        let mut guess = String::new(); // We create the variable where we later store the user-inputted guess in.

        io::stdin() // Here, we're calling the stdin() function from the io standard library that we imported at the start.
            // If we hadn't imported it, we could still call it using std::io::stdin(). Similar to Python's time.sleep()
            // stdin() returns an instance of std::io::stdin, Which is a type that represents a handle to the input of your terminal.

            .read_line(&mut guess) /* Calls the read_line() function from that handler we made above.
        We're also passing "&mut guess" as an argument to tell it where to store the string.
        what read_line() does is it takes the user input, and appends that into a string. Because it's appended it must be a mutable string.
        That's why we give it a mutable string.

        The & part means that this argument we're giving is a reference. What is a reference?
        A reference gives you a way of letting multiple parts of your code access one piece of data
        without the need of copying that data multiple times.
        references are immutable by default!

        The reason this reference is explicitly stated as mutable, is because we change it. Immutable variables can't be changed. */


        .expect("Failed to read line"); // There is always a possibility that something might go wrong. This handles that.

        /* Because we can't compare a string with an integer, we convert it so the types match.
           We also trim the value so any whitespaces get removed. Else it can't be a string.
           We once again make use of a expect() to catch any errors.

           While we do this, we also check if any error occurs, an error will occur for example when letters are inputted instead of numbers
           If that is the case, we just start again at the start of the loop.
        */
        let guess: u32 = match guess.trim().parse() { // Convert string -> integer (u32 in this case)
            Ok(num) => num, // If the parsing goes Ok, parse will return the value it produced with Ok (which we store in num). And we return that.
            Err(_) => continue, // If not, it returns Err, with the error that occured (which we store in _), and we restart at the beginning of the loop.
        }; 


        /* There are two ways we can do this.
        * println!("You guessed: {}", guess);
        or
        * println!("You guessed: {guess}"); <-- This one is easier,
        */
        println!("You guessed: {guess}");

        

        // Now, we compare the user-inputted number with the random number we generated at the start of the program.

        //       Compare
        //          V       V
        match guess.cmp(&random_number) {
            Ordering::Less => println!("You guessed too small!"),
            Ordering::Greater => println!("You guessed too big!"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }
    }
}
