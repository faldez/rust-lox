use std::io::{self, Write};

use lox::lox::Lox;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut lox = Lox::new();
    if args.len() > 2 {
        let _ = io::stdout().write(b"Usage: rust-lox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        let _ = lox.run_file(args[1].clone());
    } else {
        let _ = lox.run_prompt();
    }
}