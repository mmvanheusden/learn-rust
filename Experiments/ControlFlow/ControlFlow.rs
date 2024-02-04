#![allow(non_snake_case)]
/// Learning control flow
/// Control flow decides if a piece of code may run or not. (true/false)
fn main() {
    // If expressions
    let number = 6;
    if number < 2 { // if expressions don't have to be in () hooray!
        println!("The condition was true!")
    } else {
        println!("The condition was false :(")
    }


    // Handling multiple conditions with else if (elif)
    let number_two = 13;
    if number_two % 4 == 0 { // x % y returns the remainder. so for example 10 % 3 returns 1.
        println!("number is divisible by 4");
    } else if number % 6 == 0 {
        println!("number is divisible by 6");// <-- after this it 'exits' the 'loop'
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


    // Using if in a let statement
    let condition = true;
    /*                      'if condition == true' is the same as 'if condition'
                                   |
                                   |
                                   V              */
    let number_three = if condition { 5 } else { 6 };
    // let number_three = if condition { 5 } else { "six" } <--- This won't work, because the compiler needs a guarantee what the type will become. before calculating it.

    println!("the value of the third number is {number_three}"); // returns 5 because condition = true. If it was false it would've been 6.



    // Repeating code with loops


    /*loop {
        println!("hfehfe"); <----- Loops this forever.
    }*/

    // Now we are going to create a counter so we can choose after how many iterations we stop the loop.

    let mut counter = 0; // <-- We create a mutable counter variable. in it the count will be stored

    // We add the value we want to be returned after the loop completes.
    let result = loop {
        counter += 1; // Each time we start the loop, we add 1 to the counter

        if counter == 10 { // when the counter hits 10, we do the following
            break counter * 2; // We break the loop, and return the counter times 2 with it.
        }
    };
    // result will be stored in result.

    println!("The result is {result}");



    // Loop labels: label loops
    let mut counter_two = 0;//2: this counter starts at 0

    'counting_up: loop { // we tag this loop with label 'counting up'. we can later reference this label to stop it.
        println!("Counter: {counter_two}"); //2: We print how much this counter is
        let mut remaining = 10; //1:  We start with 10

        loop {
            println!("remaining = {remaining}"); //1: we print how much remaining
            if remaining == 9 {
                break; // We break out of this loop.
            }
            if counter_two == 2 {
                break 'counting_up // if counter is 2 we break to loop with label 'counting_up'
            }
            remaining -= 1 // 1: each time we remove 1 from the remaining variable.
        }

        counter_two += 1;
    }
    println!("End count = {counter_two}");


    // while loops
    let mut counter = 4; // start at 4.

    while counter != 0 { // if counter isn't 0, do this
        println!("Counter is at {counter}");

        counter -= 1; // each time remove 1 from the counter
    }
    // once counter hits 0 it continues.
    println!("COUNTER IS 0!!!");


    // Looping through collections
    let collection = [10,20,30,40,50,60,70,80,90];
    let mut index = 0;

    while index < 9 {
        println!("the value is: {}", collection[index]);
        index += 1;
    }
    println!("Done looping through collection.");

    index = 0;
    while index < collection.len() {
        println!("the value is: {}", collection[index]);
        index += 1;
    }
    println!("Done looping through collection.");

    for element in collection {
        println!("the value is: {}", element);
    }

    // Here we create a Range. And then loop through it.
    // Rev reverses the range. Will be covered later.
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("NUMBER IS 0!!!");
}
