use std::io::{self, Read};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

fn main() {
    enable_raw_mode().unwrap();
    for byte in io::stdin().bytes() {
        let character = byte.unwrap() as char;
        println!("{}", character);
        if character == 'q' {
            disable_raw_mode().unwrap();
            println!("Exiting...");
            break;
        }
    }
}
