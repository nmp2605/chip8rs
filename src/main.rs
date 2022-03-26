mod memory;

use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 1 {
        panic!("The location to the chip8 ROM is required.");
    }

    let rom_path: &String = &args[0];

    let contents: Vec<u8> = fs::read(rom_path)
        .expect("The file path is invalid.");
}