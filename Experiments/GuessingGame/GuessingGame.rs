use std::io;
fn main() {
    println!("Guess the number! Please give me your guess: ");

    let mut guess = String::new(); // This variable is mutable. that means we can't change it.


    // the String::new() in the guess var creation, means that it is assigned a String type.
    // This String type is growable, and encoded in UTF-8.
    // The ::new part creates a new String. It's an associated function of something, in this case String.

    io::stdin() // Here, we're calling the stdin() function from the io standard library that we imported at the start.
        // If we hadn't imported it, we could still call it using std::io::stdin(). Similar to Python's time.sleep()
        // stdin() returns an instance of std::io::stdin, Which is a type that represents a handle to the input of your terminal.

        .read_line(&mut guess) // Calls the read_line() function from that handler we made above.
        // We're also passing "&mut guess" as an argument to tell it where to store the string.
        // what read_line() does is it takes the user input, and appends that into a string. Because it's appended it must be a mutable string.
        // That's why we give it a mutable string.

        // The & part means that this argument we're giving is a reference. What is a reference?
        // A reference gives you a way of letting multiple parts of your code access one piece of data
        // without the need of copying that data multiple times.
        // references are immutable by default!


        .expect("Failed to read line");

    println!("You guessed: {guess}");
}

fn unrelated() {
    let guess2 = 5; // this one is actually not mutable. Does that mean that we immediately have to assign it a value?
    let guess3;
    guess3 = 4; // No we don't. It can be assigned later.
}
