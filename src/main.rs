use std::{
    fs::File,
    io::{BufReader, Read},
};

use crate::interpreter::Interpreter;

mod interpreter;
mod parser;
mod tokenizer;

fn main() {
    let mut buffer = String::new();
    let file = match File::open("code.txt") {
        Err(_) => {
            println!("unable to open");
            return;
        }
        Ok(f) => f,
    };

    let mut buf_reader = BufReader::new(file);
    buf_reader
        .read_to_string(&mut buffer)
        .expect("couldn't read file");

    Interpreter::interpret(buffer);
}
