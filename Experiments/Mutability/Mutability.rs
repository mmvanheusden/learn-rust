fn main() {
    // We create a mutable variable called x.
    let mut x = 3;
    println!("X equals: {x}");
    x = 6;
    println!("X is now {x}!");

    /* Running this code causes an error. That is because y is IMMUTABLE. It can't be changed.
    println!("Now, we are going to try changing an immutable variable.");
    let y = 5;
    println!("Y equals: {y}");
    y = 2;
    println!("Y is now {y}!"); */


    // Here, we are going to create a immutable variable, but without immedeatly assigning a value. 
    let guess3;
    guess3 = 4; // This is posible. However this can obviously only be done once per variable

    let mut guess5; // Also with mutable variables
    guess5 = 3;


    // We're going to create a constant. It can't be mutable, and its type must always be defined adn initialization
    const MINUTES_IN_A_WEEK: u32 = 60 * 24 * 7;
    /*                        ^
                              |
                              |
                        The data type of the constant. This must always be given when creating a constant.

    Constants *should* always be in all uppercase with underscores between words. This makes them easily recognisable.
                        */

    println!("There are {} minutes in a week.", MINUTES_IN_A_WEEK);


    // Shadowing -------------------------------------------------------
    let y = 5.0;
    let y = y + 1.0;
    // We introduce a new variable called y. We make its value 5.0.
    // After that, we recreate that variable, while adding 1.0 to the old one. So it becomes 6.0 .
    // This is called shadowing. We can use the same variable name multiple times.

    {
        let y = y * 0.5;
        println!("The value of Y in the inner scope is {}", y);
        // outputs 3. We shadow y once again, but this time, it isn't persistent and returns back to 6.0 outside of the scope.
    }

    println!("The value of Y in the regular scope is {}", y);
    // outputs 6. The modification that we did in the scope reverted.

    // Shadowing is different from mutability. We can change the type of the variable with shadowing.
    let spam = "G423^hu4i23hUY@GYdUG$YzG$uy";
    println!("Spam: {}.",spam);
    let spam = spam.len(); // Here, we change the type of spam from &str -> usize. This is possible with shadowing.
    println!("Spam has {} characters in it.", spam);


    /*
    let mut spam2 = "34h5yt834";
    spam2 = spam.len(); This won't work, because we can't just change the type of spam2. With shadowing you can, but with mutability you can't.
     */


}