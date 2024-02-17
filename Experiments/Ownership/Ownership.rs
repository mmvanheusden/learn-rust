#![allow(non_snake_case)]

mod References;
mod Slices;

/// Learning Rusts ownership system. An important aspect of Rust.
fn main() {
    /*
 * The difference between the STACK and the HEAP

 STACK:
    The stack stores and handles memory a bit like a stack of plates:
    The last plate that has been added to the pile will also be the first to leave it.
    Adding data to the stack is called pushing and removing data is called popping.
    All data on the stack needs to have a known, fixed size!
 HEAP:
    The heap is less organised. Accessing data in the heap is slower because you have to follow a pointer to get there.
    When you put something on the heap, you request a certain amount of space on the heap and the memory allocator finds a spot
    and returns a POINTER. Which points to the address of that location in memory.
    This process is called ALLOCATING on the heap. Because the size of the pointer is known and fixed the pointer can be stored on the stack, but
    where the pointer points to the size of the data is unknown.

        "Think of being seated at a restaurant. When you enter, you state
        the number of people in your group, and the host finds an empty table
        that fits everyone and leads you there. If someone in your group comes
        late, they can ask where you’ve been seated to find you."

    Pushing to the stack is faster because the allocator never has to find a place for the data, it just pushes it on top.
    Accessing data in the heap is slower because you first have to follow a pointer to get there.


        "Keeping track of what parts of code are using what data on the heap,
        minimizing the amount of duplicate data on the heap, and cleaning up
        unused data on the heap, so you don’t run out of space are all problems
        that ownership addresses. Once you understand ownership, you won’t need
        to think about the stack and the heap very often, but knowing that the
        main purpose of ownership is to manage heap data can help explain why
        it works the way it does."

    --------------------------------------------------------------------------------------------
    Ownership rules
        * Each value in rust has an OWNER
        * There can be only one owner at a time TODO: per per variable one owner at a time or in the whole program?
        * When the owner goes out of scope, the value will be dropped

 */


    // Here, S doesn't exist
    { // We open the scope.
        let S = "hello"; // we initialize S. It is valid from now on
        println!("{S}");
        // we can do stuff with S now.
    } // We close the scope.
    // The scope is now over, and S is no longer valid.

    // We create a String. This data type differs from &str, and will be stored on the heap instead of the stack.
    let mut S = String::from("Hello");
    println!("{S}");
    S.push_str(" World!"); // We append, or "push" World to the string
    println!("{S}");


    let S1 = String::from("hello");
    let S2 = S1; // Take S1, and put it in S2.
    // You would think that it completely copies S1, and puts it in S2, But that's wrong. It MOVES S1 INTO S2, and in the process,
    // It invalidates S1. This means we cannot access S1 anymore.


    // If we DO want to do copy it to s2, we can do so by CLONING.
    let s1 = String::from("hello");
    let s2 = s1.clone(); // Clone s1 into s2.
    println!("s1 = {}, s2 = {}", s1, s2);

    // This principle doesn't apply for integers, because we know it's size at compile time, and making copies on the stack is actually really easy and fast.
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);


    { // Dropping values
        let text = String::from("Taco"); // Introduce text.
        print_text(text);

        let savage = 21; // Introduce savage.
        print_integer(savage);

        fn print_text(a_string: String) { // text comes into the scope.
            println!("{}", a_string)
        } // text goes out of the scope, and gets dropped automatically. We cannot use it anymore.

        fn print_integer(an_integer: i32) { // savage enters the scope.
            println!("{}", an_integer)
        } // savage leaves the scope. Because it is Copy, nothing happens, and it isn't dropped by rust.
        // thus, we can still use savage
        println!("{}", (savage + 1));
        // But not text, because it was dropped in the print_text() function
        // println!("{}", text);
    }

    let one = gives_ownership(); // moves the string gives_ownership returns into one.
    let two = String::from("two"); // create two
    let three = takes_and_gives_back(two); // two is moved into takes_and_gives_back, which also moves its return value into three.
    fn gives_ownership() -> String {
        let some_string = String::from("one");
        some_string
    } // returns a String.
    fn takes_and_gives_back(a_string: String) -> String {
        a_string
    }

    References::main();
    Slices::main();
}
