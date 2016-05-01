#[macro_use]
extern crate chomp;

pub mod reader;
pub mod types;
pub mod printer;
pub mod eval;

use std::io;
use std::io::prelude::*;

fn main() {
    let mut input = String::new();
    loop {
        print!("λ> ");
        io::stdout().flush().expect("Cannot flush");
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {
                eval::trep(&input);
            }
            Err(error) => println!("{}", error),
        }
        input.clear();
    }
}
