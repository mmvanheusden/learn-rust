use std::collections::HashMap;

pub fn main() {
    // Let's create a Hashmap
    let mut hm1: HashMap<&str, (u8,u8,u8)> = HashMap::new();

    // Populate it with fancy colors
    hm1.insert("RED", (255,0,0));
    hm1.insert("GREEN", (0,255,0));
    hm1.insert("BLUE", (0,0,255));

    // 255 + 255 = 510
    let mix_of_red_and_blue: u32 = hm1.get("RED").unwrap().0 as u32 + hm1.get("BLUE").unwrap().2 as u32;
    println!("Red + blue = {mix_of_red_and_blue}");


    struct Color {
        R: u8,
        G: u8,
        B: u8,
    }

    let white: Color = Color {
        R: hm1.get("RED").unwrap().0,
        G: hm1.get("GREEN").unwrap().1,
        B: hm1.get("BLUE").unwrap().2,
    };

    println!("\nWhite:\nR: {}\nG: {}\nB: {}", white.R, white.G, white.B);


    // Let's handle errors

    //                      Takes the returned Option       When the unwrapping fails, Just make the color black (rgb: 0,0,0)
    //                                             V        V
    let girl_color = hm1.get("PURPLE").copied().unwrap_or((0,0,0));

    if girl_color == (0,0,0) {
        println!("Oh no, the color is black!");
    } else {
        println!("We're good!");
    }

    // Looping through a hashmap
    for (key, value) in &hm1 {
        println!("{key}: RGB{:?}", value);
    }


    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // From now on, field_name and field_value are dropped.

    // With hashmap.entry(), we can check if a key exists, and if it doesn't, add it.
    //                                                             138u8 is the same as 138 as u8
    let purple = hm1.entry("PURPLE").or_insert((138u8, 43u8, 226u8));

    println!("{}", purple.0);
}