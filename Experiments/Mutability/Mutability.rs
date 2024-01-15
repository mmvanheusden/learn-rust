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


    // We're going to create a constant. It can't be mutable, and its type must always be defined.
    const MINUTES_IN_A_WEEK: u32 = 60 * 24* 7
}