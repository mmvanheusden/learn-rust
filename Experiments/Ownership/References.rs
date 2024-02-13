/// References
pub fn main() {
    /* But what if we don't want a value to move into a function and then just dissapear?
    Then you can use a REFERENCE.
    A reference is like a pointer. It points to a variable that is owned by
     */


    let s1 = String::from("hello"); // new string
    let len = calculate_length(&s1); // We give a POINTER to s1 with the function. It REFERS to s1, but does not take ownership.

    println!("The length of '{}' is {}.", s1, len);


    /* We cannot change a borrowed variable.
    fn main() {
        let s = String::from("hello");

        change(&s);
    }

    fn change(some_string: &String) {
        some_string.push_str(", world");
    }
     */

    // To solve that, we make everything mutable!!
    let mut s = String::from("hello");
    change(&mut s);
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r4 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point
    // We can now borrow it as mutable again.

    let r3 = &mut s; // no problem
    println!("{}", r3);

}

fn calculate_length(s: &String) -> usize { // s is a reference to String
    s.len()
} // s leaves the scope, but because it is only a reference, (we dont own it), it isn't dropped.
// This is called BORROWING.