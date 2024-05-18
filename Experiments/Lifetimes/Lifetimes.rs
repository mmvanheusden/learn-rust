#![allow(non_snake_case)]
// mod OutOfScope;

/// Lifetimes.
fn main() {
    // OutOfScope::main(); -- This crashes the app.
    // Now we can obviously fix the above by putting everything in the main scope. This is not anything special
    {
        // Here, x has a lifetime that is bigger than r's lifetime.
        let x = 5;

        let r = &x;

        println!("r: {}", r);
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_lifetimes(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// This function contains a borrowed value, but the function signature does not say whether it is borrowed from x or y. This is (apparently) bad?
// Rust can't tell if the borrowed value that is returned will be a borrowed x, or a borrowed y. This makes sense because we don't know that yet

/*fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}*/

/**
To fix this, we add a generic lifetime to each parameter.

**This is how lifetimes are annotated:**
```rust
let a: &i32;        // This just a regular reference.
let b: &'a i32;     // This is a reference with an explicit lifetime
let c: &'a mut i32; // Same as above but mutable
```

**/
fn a() {} // This is here just for the stupid doc string error to go away.

// Now, let's integrate what we learned with our 'longest' function.
//
//                         V
fn longest_with_lifetimes<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// TODO: Finish this.
