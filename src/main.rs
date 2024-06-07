use core::panic;
use parser::ast_printer::ASTprinter;
use parser::expression::{self, Expression};
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

fn loxerror(line: usize, message: String) {
    report(line, String::new(), message)
}

fn lox_parser_error(token: Token, message: String) {
    match token.token_type {
        TokenType::Eof => report(token.line, "at end".to_string(), message),
        _ => report(token.line, format!("at '{}'", token.lexame), message),
    }
}

fn run(code: String) {
    println!("{}", code);
    let mut scanner = Scanner::new(code);
    let tokens: Vec<Token> = scanner.scan_tokens();
    let mut parser: Parser = Parser::new(tokens);
    match parser.parse() {
        Some(expression) => {
            let ast = ASTprinter::new();
            ast.print_tree(expression);
        }
        None => {
            println!("There was an error while parsing")
        }
    }
}

fn test_ast_tree() {
    let expression: Expression = Expression::Binary {
        left: Box::new(Expression::Unary {
            operator: Token::new(
                TokenType::Minus,
                "-".to_string(),
                0,
                LiteralValue::Float(1.0),
            ),
            right: Box::new(Expression::Literal {
                value: LiteralValue::Float(123.0),
            }),
        }),
        operator: Token::new(TokenType::Star, "*".to_string(), 0, LiteralValue::None),
        right: Box::new(Expression::Grouping {
            expression: Box::new(Expression::Literal {
                value: LiteralValue::Float(45.67),
            }),
        }),
    };
    let ast_printer = ASTprinter::new();
    let result = ast_printer.print_tree(expression);
    println!("{}", result);
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

    let ast_test_key = &"ast";
    if args.len() > 2 {
        println!("Usage: lox [source]");
    } else if args.len() == 2 && args[1] != ast_test_key.to_string() {
        let _ = run_file(&args[1]);
    } else if args.len() == 2 && args[1] == ast_test_key.to_string() {
        test_ast_tree();
    } else {
        let _ = run_prompt();
    }
}
