fn main() {
    // Let's say we have a number in a string
    let price: &str = "5.00";
    // Now, we want to convert it into a float
    let price: f32 = price.parse().expect("Not a string");
    //          ^
    // If we don't add the f32 annotation to the code, rust will get confused, because it doesn't know which type it should change to.
    println!("{price}");


    /* Rust has four *Scalar types*. integers, floating-point numbers, Booleans and characters.
    * Integers:
      An integer is a number, without a fractional component. Like 3, 10, 587943, or 23432.

      -----Integer variants-----
      Length	Signed	Unsigned
      8-bit 	i8	    u8
      16-bit	i16	    u16
      32-bit	i32     u32
      64-bit	i64 	u64
      128-bit	i128	u128
      arch	    isize	usize
      --------------------------
      So as you can see, there are two variants: signed and unsigned integers. They start with either i or u; i for signed and u for unsigned.
      Signed and unsigned refers to if the value can be negative or not.
      *When it's safe to assume the number will never be negative, you can make it unsigned.*

      For example:
        Allowed:
            Signed: -453242
            Unsigned 423

        Disallowed:
            Signed: 432423.4
            Unsigned: -4432

      The longer the length, the more it can store. Additionally, the isize, and usize depends on your computers architecture. 32-bit or 64-bit.


    * Floating-point numbers:
      The same as an integer, but with decimal points. For example: 3.4, 2.1, 1045.1, 5.0

      The default type is f64, but f32 can be used.
        let x = 2.0; // f64

        let y: f32 = 3.0; // f32

    * Mathematical operations:
      Rust supports a variety of mathematical operations on numbers:

        // addition
        let sum = 5 + 10;
    
        // subtraction
        let difference = 95.5 - 4.3;
    
        // multiplication
        let product = 4 * 30;
    
        // division
        let quotient = 56.7 / 32.2;
        let truncated = -5 / 3; // Results in -1
    
        // remainder TODO: explain and remember xD

        let remainder = 43 % 5;

    * Booleans:
      Booleans are either True, or False
      Their type doesn't have to be explicitly annotated
        let t = true;
        let f: bool = false; // with explicit type annotation
    
    * Characters:
      The character type represents a single character.
      **THEY MUST HAVE SINGLE QUOTES**
      Example:
        let c = 'z';
        let z: char = 'ℤ'; // with explicit type annotation
        let heart_eyed_cat = '😻';
      Special characters like emojis are also supported.
    */

    /*
    Compound Types:
      * Tuples
        A tuple is a general way of grouping together several values into one variable.
        Tuples have a fixed length. Once they are created they cannot be resized.
        Normally the types are automatically added but they can be explicitly stated.

          let tup: (i32, f64, u8) = (500, 6.4, 1); // with explicit type annotation
          let tup2 = (33.33, 2, -5); // Rust will do the types for you.

        We can take those 3 values and create a separate variable for each one.

          let (a, b, c) = tup;
          println("The second value in the tuple is {b}!");


      //TODO: Write down all data types
     */
}
