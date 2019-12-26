use std::io::{self, Write};

fn main() {
    // We will start by storing all inputted text to a single large String type
    let mut text = String::new();

    // Clear terminal screen
    print!("{}[2J", 27 as char);
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read text");

    println!("You entered: {}", text);
}
