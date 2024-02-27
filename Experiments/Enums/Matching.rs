//ref: https://nl.wikipedia.org/wiki/Nederlandse_gulden#Vanaf_1816
// create types of coins
enum Coin {
    Stuiver(u16),
    Dubbeltje(bool),
    Kwartje,
    Rijksdaalder
}


fn value_in_gulden(coin: Coin) -> u8 {
    match coin {
        Coin::Stuiver(year) => {
            println!("This Stuiver was produced in {}", year);
            5
        },
        Coin::Dubbeltje(limited) => {
            if limited {
                println!("This coin is limited edition!");
                return 25 // Worth more because limited edition
            }
            10
        },
        Coin::Kwartje => {
            println!("Easter egg!");
            25
        },
        Coin::Rijksdaalder => 250,
    }
}

#[allow(clippy::manual_map)]
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // if None is given, give that shit straight back
        Some(i) => Some(i + 1) // else, do the calculation
        // This prevents errors.
    }
}

fn roll_dice(roll: u8) {
    match roll {
        2 => println!("Rolled {roll}: You are lucky!"),
        7 => println!("Rolled {roll}: You are very unlucky!"),
        // A match must always have an outcome.
        _ => println!("Rolled {roll}: nothing happens."),
    }
}

pub fn main() {
    /*!
    Rust has an extremely powerful control flow construct named `match`. It allows you to compare a value against a series of **patterns** and then execute code based on the outcome.
    Patterns can be made of many things.
   */
    println!("50 Stuivers are worth ƒ{},-.", (value_in_gulden(Coin::Stuiver(1968)) * 50));
    println!("15 Dubbeltjes are worth ƒ{},-.", (value_in_gulden(Coin::Dubbeltje(true)) as u32 * 15));// convert to u32 else won't fit in u8
    println!("15 Dubbeltjes are worth ƒ{},-.", (value_in_gulden(Coin::Dubbeltje(false)) * 15));

    let three = Some(3);
    let four = plus_one(three);
    let nothing = plus_one(None);
    if nothing.is_none() {println!("None!");};
    if four.is_none() {println!("None!");};
    roll_dice(3);
    roll_dice(7);
    roll_dice(2);
    roll_dice(8);
}