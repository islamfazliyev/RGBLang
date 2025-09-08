use crate::{cli_ui::cli_ui, lexer::lexer, parser::Parser};

mod lexer;
mod parser;
mod tokens;
mod cli_ui;
use cli_ui::CliError;


fn main()
{
    println!("RGBLang");
    run();
}

fn run()
{
    loop {
        println!("");
        let input = match cli_ui() {
            Ok(input) => input,
            Err(CliError::ReadError(e)) => {
                eprintln!("Read error: {}", e);
                return;
            }
        };

        if input.trim() == "exit" || input.trim() == "quit" {
            println!("Goodbye! 👋");
            break;
        }
        
        let tokens = match lexer(&input) {
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
    
}