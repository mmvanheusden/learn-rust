pub fn main() {
    let config_max = Some(3u8);


    #[allow(clippy::single_match)]
    // This:
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // Can also be written as:
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    // This is simpler.
}