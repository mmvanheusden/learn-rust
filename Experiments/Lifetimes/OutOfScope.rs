pub fn main() {
    let r;

    {
        let x = 5;// We create x that will be dropped later.
        r = &x; // We create a reference to x.
        // Because x gets out of scope, the reference to it (which is stored in r) becomes invalid. x doesn't live long enough.
    }
    // Where's x? gone. r breaks.

    println!("r: {}", r);
}