use crate::{cli_ui::cli_ui, lexer::lexer, parser::Parser};
use clap::{Arg, Parser as otherParser};
use std::fs;

mod lexer;
mod parser;
mod tokens;
mod cli_ui;
use cli_ui::CliError;

#[derive(otherParser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    file: Option<String>,
}

fn main()
{
    println!("RGBLang");
    let args = Args::parse();
    if let Some(file_path) = args.file {
        run_file(&file_path);
    } else {
        
        run();
    }
}

fn run_file(path: &str) {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Dosya okunamadı: {}", e);
            return;
        }
    };

    let tokens = match lexer(&content) {
        Ok(t) => t,
        Err(err) => {
            eprintln!("Lexer error: {}", err);
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    while let Some(_) = parser.current_token() {
        if let Err(err) = parser.sense() {
            eprintln!("Parser error: {:?}", err);
            break;
        }
    }
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