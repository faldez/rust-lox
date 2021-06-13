use std::{error::Error, fs, io::{self, BufRead, Read, Write}};

use crate::scanner::Scanner;

pub struct Lox {
    had_error: bool
}

impl Lox {
    pub fn new() -> Self {
        Lox{
            had_error: false,
        }
    }
    pub fn run_file(&self, path: String) -> Result<(), Box<dyn Error>> {
        let source = fs::read_to_string(path)?;
        self.run(source);

        Ok(())
    }

    pub fn run_prompt(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            io::stdout().write(b">").expect("should write");
            io::stdout().flush().expect("should flush");

            let line = io::stdin().lock().lines().next().unwrap().unwrap();
            self.run(line);
            self.had_error = false;
        }
    }

    pub fn run(&self, source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for token in tokens {
            io::stdout().write(token.to_string().as_bytes()).expect("should write");
            io::stdout().write(b"\n").expect("should write");
            io::stdout().flush().expect("should flush");
        }
    }

    pub fn error(&mut self, line: i64, message: String) {
        self.report(line, "".to_string(), message)
    }

    pub fn report(&mut self, line: i64, place: String, message: String) {
        log::error!("[line {}] Error {}: {}", line, place, message);
        self.had_error = true;
    }
}