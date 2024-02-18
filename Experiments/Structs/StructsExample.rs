// ref: https://en.wikipedia.org/wiki/ISO_216#Dimensions_of_A,_B_and_C_series

/// Contains the width and height of a rectangle.
#[derive(Debug)] // If we want to print the whole contents of a struct, we must opt-in to that using this.
struct Rechthoek {
    breedte: f32,
    hoogte: f32,
}

pub fn main() {
    // A4 paper, dimensions in mm
    let A4_width = 210;
    let A4_height = 297;

    // A3 paper, dimensions in mm
    let A3_maat = (297, 420);

    // C4 paper, dimensions in inches
    let C4 = Rechthoek {
        breedte: 9.0,
        hoogte: 12.8,
    };

    println!(
        "De oppervlakte van een A4'tje is {} mm².",
        oppervlakte(A4_width, A4_height) // calculate the area
    );

    println!(
        "De oppervlakte van een A3 vel is {} cm².",
        oppervlakte2(A3_maat) as f32 * 0.01 // comvert square milimeter to square centimeter
    );

    println!(
        "De oppervlakte van een C4 vel is ≈{} in², en ≈{} cm²",
        oppervlakte3(&C4).round(),
        (oppervlakte3(&C4) * 6.4516).round() // convert square inches to square centimeters. 1 in2 = 6.4516 cm2.
    );

    // Print the whole C4 struct contents, for debugging
    println!("The content of the C4 variable is: '{:?}'", C4);
}

/// Calculate area using two, given values: width and height.
fn oppervlakte(breedte: u32, hoogte: u32) -> u32 {
    breedte * hoogte
}

/// Calculate area using a tuple, containing the width and height.
fn oppervlakte2(maat: (u32, u32)) -> u32 {
    maat.0 * maat.1
}

/// Calculate area with a given Rechthoek.
fn oppervlakte3(vierkant: &Rechthoek) -> f32 {
    vierkant.hoogte * vierkant.breedte
}