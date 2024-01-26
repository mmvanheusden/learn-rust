use std::io;

/// Playing around with tuples
fn main() {
    // let mut coords = (0,0,0);
    let mut x = String::new();
    let mut y = String::new();
    let mut z = String::new();


    // Ask for coordinates
    println!("enter x coord! ");
    io::stdin().read_line(&mut x).expect("Failed to read line");

    println!("enter y coord! ");
    io::stdin().read_line(&mut y).expect("Failed to read line");

    println!("enter z coord! ");
    io::stdin().read_line(&mut z).expect("Failed to read line");


    // Convert the strings to 32-bit integers
    let x: u32 = x.trim().parse().expect("Not a number");
    let y: u32 = y.trim().parse().expect("Not a number");
    let z: u32 = z.trim().parse().expect("Not a number");

    // Put them all in a tuple
    let mut coords = (x, y, z);


    // print them
    println!("Your coords are:\nX: {}\nY: {}\nZ: {}", coords.0, coords.1, coords.2);

    // Types can't be changed after initializing.
    //coords = ("ten", "three", "fifteen");

    coords = (coords.0 * 2, coords.1 * 2, coords.1 * 2);

    println!("The coords are now:\nX: {}\nY: {}\nZ: {}", coords.0, coords.1, coords.2);

    println!("{}", coords.0.pow(coords.2/5+10))
}