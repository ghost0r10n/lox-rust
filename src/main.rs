use core::panic;
use interpreter::interpreter::Interpreter;
use parser::parser::Parser;
use scanner::scanner::Scanner;
use scanner::token::Token;
use scanner::token_type::TokenType;
use std::fmt::Result;
use std::fs::File;
use std::io::{prelude::*, stdin};
use std::{env, io, usize};
use utils::literal_value::LiteralValue;

pub mod interpreter;
pub mod parser;
pub mod scanner;
pub mod utils;

fn report(line: usize, whr: String, message: String) {
    println!("[line {}] Error {}: {}", line, whr, message)
}

fn report_panic(line: usize, whr: String, message: String) {
    println!("[line {}] Runtime Error {}: {}", line, whr, message)
}

fn loxerror(line: usize, message: String) {
    report(line, String::new(), message)
}

fn lox_parser_error(token: Token, message: String) {
    match token.token_type {
        TokenType::Eof => report(token.line, "at end".to_string(), message),
        _ => report(token.line, format!("at '{}'", token.lexame), message),
    }
}
fn lox_runtime_error(token: Token, message: String) -> LiteralValue {
    match token.token_type {
        TokenType::Eof => report_panic(token.line, "at end".to_string(), message),
        _ => report_panic(token.line, format!("at '{}'", token.lexame), message),
    }
    return LiteralValue::None;
}

fn run(code: String, is_ast: bool) {
    let mut scanner = Scanner::new(code);
    let tokens: Vec<Token> = scanner.scan_tokens();

    let mut parser: Parser = Parser::new(tokens);
    match parser.parse() {
        Ok(statements) => {
            let mut interpreter: Interpreter = Interpreter::new();
            interpreter.interpet(statements);
        }
        Err(error) => {
            parser.sync();
            lox_parser_error(error.token, error.message)
        }
    }
}

//From source mode
fn run_file(path: &String, is_ast: bool) -> Result {
    let file_result = File::open(path);
    let mut file_result: File = match file_result {
        Ok(file) => file,
        Err(_) => panic!("rlox:: Problem opening the file"),
    };
    let mut content: String = String::new();

    match file_result.read_to_string(&mut content) {
        Ok(_) => Ok(run(content, is_ast)),
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
                run(input, false);
            }
            Err(_) => panic!("rlox:: Problem reading input"),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let ast_test_key = &"ast";
    if args.len() > 3 {
        println!("Usage: lox [source]");
    } else if args.len() == 2 {
        let _ = run_file(&args[1], false);
    } else if args.len() == 3 && args[2] == ast_test_key.to_string() {
        let _ = run_file(&args[1], true);
    } else {
        let _ = run_prompt();
    }
}
