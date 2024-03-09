#![allow(non_snake_case)]

mod ReturningResults;

use std::fs::File;
use std::os::unix::fs::PermissionsExt; // This probably doesn't on windows
use std::io::ErrorKind;

/// Handling errors in Rust
fn main() {
    // We can intentionally panic with panic!()
    //panic!("oops");

    let v = vec![1, 2, 3];
    // v[99]; // Oh no! index 99 doesn't exist! BAM CRASH


    // Let's handle potential errors
    let readme_file_result  = File::open("README.md");

    let readme_file = match readme_file_result {
        Ok(file) => {
            println!("DEBUG: The file was correctly read. \nThe permission mode of the file is: {:?}", file.metadata().unwrap().permissions().mode());
            file
        }, // return the file
        Err(error) => {
            panic!("The file had a problem opening.\nError: {:?}", error)
        }
    };

    // Let's extend this a little bit.
    let cargo_toml_file_result = File::open("carg.toml");

    let cargo_toml = match cargo_toml_file_result {
        Ok(file) => {
            println!("The file was correctly read. \nThe permission mode of the file is: {:?}", file.metadata().unwrap().permissions().mode());
            file
        }, // return the file
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("The file was not found. Maybe you made a typo? ;) anyways i'll create it for you but you're not getting away with it next time");
                match File::create("carg.toml") {
                    Ok(fc) => {
                        println!("Done!");
                        fc // Return the file
                    },
                    Err(error) => {
                        panic!("Something bad happened! We can't create a file. ERROR: {error}")
                    }
                }
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        }
    };

    ReturningResults::main()
}
