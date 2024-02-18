#![allow(non_snake_case)]

mod StructsExample;
mod Methods;

/// Learning Structs

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct RGB(u8, u8, u8);

struct Coordinates(i32, i32, i32);

fn main() {
    /*!
    Structs are similar to tuples. They hold multiple related values.
    the values can be different types. In structs you name them also.
    to define a struct, we use `struct` and name the struct.
     */

    // Now we are going to add a new user.
    // If we want to change a value, we must make the entire struct mutable.
    let mut admin_user = User {
        active: true,
        username: String::from("administrator"),
        email: String::from("webmaster@example.com"),
        sign_in_count: 1,
    };

    // now we retrieve a value from a struct
    println!("The email of the webmaster is: {}", admin_user.email);
    // Change the email of the administrator
    admin_user.email = String::from("former-webmaster@example.com");
    println!("The email of the webmaster has been changed to: {}", admin_user.email);


    // Lets create another webmaster!
    let mut admin_user2 = User {
        email: String::from("webmaster2@example.com"),
        username: String::from("admininstrator2"),
        ..admin_user // Fill the rest with the data from the first admin user.
    };

    println!("User {} has been created with email {}!", admin_user2.username, admin_user2.email);
    deactivate_user(&mut admin_user2);
    if admin_user2.active {
        println!("Admin 2 is active!");
    } else {
        println!("Admin 2 is inactive!");
    }

    let honeydew = RGB(240, 255, 240);
    let base_coordinates = Coordinates(-432432, 60, 5345);

    println!(
        "My base is located at\nX: {}\nY: {}\nZ: {}",
        base_coordinates.0, base_coordinates.1, base_coordinates.2
    );
    println!(
        "The RBG values of the 'honeydew' color are:\nRed: {}\nGreen: {}\nBlue: {}",
        honeydew.0, honeydew.1, honeydew.2
    );
    StructsExample::main();
    Methods::main();
}

fn add_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn deactivate_user(user: &mut User) {
    user.active = false;
}
