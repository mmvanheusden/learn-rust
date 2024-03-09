pub fn main() {
    let string1 = String::from("Hello ");
    let string2  = String::from("there!");

    // Rust "converts" a &String into a &str
    let string3 = string1 + &string2;
    // An easier way is to use the `format!()` macro.

    let string4 = String::from("Hello ");
    let string5  = String::from("there!");
    let s = format!("{string4}{string5}");
    println!("{s}");


    // We can't slice a String. Instead, we use a reference

    let slice_of_string3 = &string3[..5];
    println!("{slice_of_string3}");


    let fact = "rust is awesome!";
    for character in fact.chars() {
        println!("{character}");
    }
    for character in fact.bytes() {
        println!("{character}");
    }
}