#![allow(non_snake_case)]
mod Util;

/// This is my awful code for school. Please don't look at it
fn main() {
    let mut punten: Vec<(f64, f64)> = vec![]; // We store the x,y points here
    const HOOGTE: f32 = 1.0e3; // The height the kogel is dropped at. in metres
    let mut DELTA_HOOGTE: f32 = 1.0; // the difference in height
    const GRAVITATIECONSTANTE_NL: f32 = 9.81; // Gravitatieconstante
    const MASSA: f32 = 2.0; // mass of kogel in kg

    while DELTA_HOOGTE >= 0.0 {
        println!(
            "x: {DELTA_HOOGTE}, y: {}",
            Util::kogelSnelheidBerekenen(HOOGTE, DELTA_HOOGTE, MASSA, GRAVITATIECONSTANTE_NL)
        );
        punten.push((
            DELTA_HOOGTE as f64,
            Util::kogelSnelheidBerekenen(HOOGTE, DELTA_HOOGTE, MASSA, GRAVITATIECONSTANTE_NL),
        ));
        DELTA_HOOGTE -= 0.001; // each step is 0.001.
    }
    Util::plot(punten).expect("ERROR!");
}
