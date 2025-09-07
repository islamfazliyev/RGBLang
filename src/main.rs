use crate::{lexer::{lexer}, parser::Parser};
use std::{io::{self}, };

mod lexer;
mod parser;
mod tokens;

fn main()
{
    let stdin = io::stdin();
    let mut input = String::new();
    
    if let Err(err) = stdin.read_line(&mut input) {
        eprintln!("Failed to read line: {}", err);
        return;
    }
    
    let tokens = match lexer(&input.trim()) {
        Ok(tokens) => {
            tokens
        }
        Err(err) => {
            eprintln!("Lexer error: {}", err);
            return;
        }
    };
    

    let mut parser = Parser::new(tokens);

    while parser.current_token().is_some() {
        match parser.sense() {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Parser error: {:?}", err);
                break;
            }
        }
    }
}