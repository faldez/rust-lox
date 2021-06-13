use std::{
    env,
    io::{self, Write},
    process,
};

use lox::Lox;

#[macro_use]
extern crate lazy_static;

mod lox;
mod scanner;
mod token;
mod token_type;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lox = Lox::new();
    if args.len() > 2 {
        let _ = io::stdout().write(b"Usage: rust-lox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        let _ = lox.run_file(args[1].clone());
    } else {
        let _ = lox.run_prompt();
    }
}
