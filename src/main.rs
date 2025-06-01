use std::io;
use std::fs;

pub mod generator;
pub mod game;

fn main() {
    let files = fs::read_dir("src").unwrap();
    for entry in files {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("rs") && path.file_name().and_then(|s| s.to_str()) != Some("main.rs") {
            println!("{}", path.display());
     }
    }
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    if input.trim() == "game" {
        game::run();
    } else if input.trim() == "generator" {
        generator::run();
    }
}