pub fn main() {
    let r;

    {
        let x = 5;// WE create x that will be dropped later.
        r = &x; // We create a reference to x.
        // Because x gets out of scop, the reference to it (which is stored in r) becomes broken. x doesn't live long enough.
    }

    println!("r: {}", r);
}