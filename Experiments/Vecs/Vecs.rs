#![allow(non_snake_case)]

mod EncodedText;
mod HashMaps;

use std::io;

#[allow(clippy::manual_unwrap_or)]
/// Vector !!!!!!!!!!1
fn main() {
    // vectors are a better way of creating lists.
    // lets create a new vector
    let vec_a: Vec<i32> = Vec::new();
    // There's an easier way (using a macro) to create a vector
    let vec_b = vec![1];
    // we can also populate this
    let vec_c = vec![1,4,68,8,9];

    // ofcourse we can also create a mutable vector
    let mut vec_d = vec![3,6,8];
    vec_d.push(5); //todo can we push mutiple at once??
    // [3,6,8,5]
    // lets remove the last one of the array
    vec_d.pop();
    // [3,6,8]


    // lets print out a chosen element
    let mut vec_e = vec![1,2,3,4,5,6,7,8,9,10];
    vec_e.reverse(); // ;)
    // [10,9,8,7,6,5,4,3,2,1]
    //  0  1 2 3 4 5 6 7 8 9
    /*     ^
           |
           |
     */

    let second = &vec_e[1];
    println!("{second}");

    println!("Which index do you want to get from the vector?");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");

    let choice1: usize = match choice.trim().parse() { // remove the shit from the string and convert(parse) to integer
        Ok(num) => num, // if a number was typed store it
        Err(_) => panic!("put in a number idiot"), // else
    };

    let choice: Option<&i32> = vec_e.get(choice1.clone());
    match choice {
        Some(third) => println!("Element nummer {} is {third}", choice1),
        None => panic!("er is geen index {} dumkop", choice1),
    }


    EncodedText::main();
    HashMaps::main();
}
