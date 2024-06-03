use core::panic;
use scanner::scanner::Scanner;
use scanner::token::Token;
use std::fmt::Result;
use std::fs::File;
use std::io::{prelude::*, stdin};
use std::{env, io, usize};

pub mod parser;
pub mod scanner;

fn report(line: usize, whr: String, message: String) {
    eprintln!("[line {}] Error {}: {}", line, whr, message)
}

fn loxerror(line: usize, message: String) {
    report(line, String::new(), message)
}

fn run(code: String) {
    println!("{}", code);
    let mut scanner = Scanner::new(code);
    let tokens: Vec<Token> = scanner.scan_tokens();
    for token in tokens {
        println!("{}", token);
    }
}

//From source mode
fn run_file(path: &String) -> Result {
    let file_result = File::open(path);
    let mut file_result: File = match file_result {
        Ok(file) => file,
        Err(_) => panic!("rlox:: Problem opening the file"),
    };
    let mut content: String = String::new();

    match file_result.read_to_string(&mut content) {
        Ok(_) => Ok(run(content)),
        Err(_) => panic!("rlox:: Problem reading content of the file"),
    }
}

//From prompt shell
fn run_prompt() -> Result {
    loop {
        print!(">> ");
        io::stdout().flush().expect("rlox:: Failed to flush stdout");
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 1 {
                    break Ok(());
                }
                run(input);
            }
            Err(_) => panic!("rlox:: Problem reading input"),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: lox [source]");
    } else if args.len() == 2 {
        let _ = run_file(&args[1]);
    } else {
        let _ = run_prompt();
    }
}
