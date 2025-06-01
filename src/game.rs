use std::io;

pub fn run() {
    println!("Write something");

    let mut text = String::new();

    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read line");
    println!("You typed in: {}", text)
}