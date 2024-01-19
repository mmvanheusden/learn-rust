use colored::Colorize;

fn main() {
    /* Printing colored text in rust is easy. There are two ways
    * 1. Print using hexadecimal escape sequences.

    * 2. Using the 'colored' crate.
    */

    // Here, we use use the boring method.
    println!("This text is colored using \x1b[93mhexadecimal escape sequences\x1b[0m ✨✨✨\n");

    // Here, we use the fancy crate method. Way user-friendlier if you ask me!
    println!(
        "This text is both {}{}{}{}{}{}{}{}{} and {}{}{}{}{}{}{} using the {}{}{}{}{}{}{} crate. ",
        // formatted (in random formatting)
        "f".strikethrough(),
        "o".bold(),
        "r".underline(),
        "m".italic(),
        "a".bold().italic(),
        "t".strikethrough(),
        "t".bold(),
        "e".underline(),
        "d".italic(),

        // colored (in random colors)
        "c".green(),
        "o".red(),
        "l".yellow(),
        "o".bright_blue(),
        "r".blue(),
        "e".purple(),
        "d".white(),

        // colored (random formatting + colors)
        "c".blue().bold(),
        "o".green().italic().bold(),
        "l".red().underline().bold(),
        "o".yellow().italic().bold(),
        "r".bright_blue().strikethrough().bold(),
        "e".purple().bold(),
        "d".white().italic().bold()
    );
}