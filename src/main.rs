use std::io;

fn main() {
    println!("Here is your pen. Begin inputting text...");
    // We will start by storing all inputted text to a single large String type
    let mut text = String::new();

    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read text");

    println!("You entered: {}", text);
}
