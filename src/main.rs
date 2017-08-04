#![feature(inclusive_range)]

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod bfr;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        panic!("Pass the Brainfuck file path as the only argument");
    } else {
        let path = Path::new(&args[1]);
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => panic!("Failed to open '{}': {}", display, why.description()),
            Ok(file) => file
        };

        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Err(why) => panic!("Failed to read '{}': {}", display, why.description()),
            Ok(_) => bfr::rt::BrainfuckMachine::new_stdio(content.as_bytes()).execute_all()
        }
    }
}
