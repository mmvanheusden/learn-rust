use std::fs;
use std::fs::File;
use std::io::{self, Read};

pub fn main() {
    //                     We use unwrap() here to use the String inside the Result our function returned. If an error happened it will panic accordingly.
    //                                                         V
    let readme_contents = read_file("README.md").unwrap();
    println!("The contents of the README are:\n {readme_contents}");


    // Exact same thing but cool
    let readme_contents = read_file_cool("README.md").unwrap();
    println!("The contents of the README are:\n {readme_contents}");


    // Exact same thing, but even cooler
    let readme_contents = read_file_quick("README.md").unwrap();
    println!("The contents of the README are:\n {readme_contents}");


    let really_green = Color::new(0,255,0);

    println!("Really green G: {}", really_green.getG());
}


// This returns a Result<a,b> a is the content of the file read, and b is a possible error.
fn read_file(file: &str) -> Result<String, io::Error> {
    // First read the file.
    let result = File::open(file);

    // Then handle any errors that might have occurred
    let mut file = match result {
        Ok(file) => file, // if no errors occurred store the file in file
        Err(e) => return Err(e)
    };

    // in here we store the contents of the file we just read
    let mut contents = String::new();

    // attempt to read the file. If errors happen, we handle that
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),// if no errors occurred we write it.
        Err(e) => Err(e) // else return the error
    }
}


// This is the same, but shorter
fn read_file_cool(file: &str) -> Result<String, io::Error> {
    // first read the file. The ? means, do the operation, and if something happens return an error(Err(e)) and else return the result(Ok(r).
    let mut file = File::open(file)?; // ?

    // store contents here
    let mut contents = String::new();

    // write the data in contents. Errors are handled same as first line.
    file.read_to_string(&mut contents)?; // ?

    // Then return the contents within an Ok(). This will work 100& because we have already outruled any other errors.
    Ok(contents)
}


// SHORTER
fn read_file_cooler(file: &str) -> Result<String, io::Error> {
    // Init contents var
    let mut contents = String::new();

    // read file
    File::open(file)? // <- handle errors
        // read contents
        .read_to_string(
            &mut contents // write contents
        )?; // <- handle errors

    Ok(contents) // <- If no errors occured return the contents within an Ok()
}

// SHOERR+RROURURPUYGIOP
fn read_file_quick(file: &str) -> Result<String, io::Error> {
    // This litterly does everything we just made in one function.
    fs::read_to_string(file)
    // Looking at the source code of fs::read_to_string() this looks more reliable to use.
}


pub struct Color {
    r: u16,
    g: u16,
    b: u16,
}

impl Color {
    pub fn new(r: u16, g: u16, b: u16) -> Color {
        if r > 255 || g > 255 || b > 255 {
            panic!("A color can't be more than 255!")
        }
        Color { r, g, b }
    }

    pub fn getR(&self) -> u16 {
        self.r
    }
    pub fn getG(&self) -> u16 {
        self.g
    }
    pub fn getB(&self) -> u16 {
        self.b
    }
}