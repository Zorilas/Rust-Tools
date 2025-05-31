use std::io;
use rand::Rng;

fn main() {
    let options: [&'static str; 57] = [
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M",
        "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m",
        "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
        "=", "!", ".", "?", "%"
    ];

    println!("Please tell how many digits your password should have!");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let length: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    let mut rng = rand::thread_rng();
    let mut password = String::new();

    for _ in 0..length {
        let idx = rng.gen_range(0..options.len());
        password.push_str(options[idx]);
    }

    println!("Generated password: {}", password);
}
