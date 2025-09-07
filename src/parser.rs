use std::error;

use crate::{ tokens::Tokens};
use colored::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedToken { expected: Tokens, found: Option<Tokens> },
    UnkownType { type_name: String},
}

pub struct Parser {
    tokens: Vec<Tokens>,
    pos: usize,
    output: Vec<String>,
}


impl Parser {
    pub fn new(tokens: Vec<Tokens>) -> Self {
        Parser {
            tokens,
            pos: 0,
            output: Vec::new(),
        }
    }

    pub fn current_token(&self) -> Option<&Tokens> {
        self.tokens.get(self.pos)
    }

    fn eat(&mut self, token_type: Tokens) -> Result<(), ParseError> {
        if let Some(tok) = self.current_token() {
            if *tok == token_type {
                self.pos += 1;
                Ok(())
            }
            else {
                Err(ParseError::UnexpectedToken {
                    expected: token_type,
                    found: Some(tok.clone()),
                })
            }
        }
        else {
            Err(ParseError::UnexpectedToken {
                expected: token_type,
                found: None,
            })
        }
    }

    fn handle_token(&mut self, token: Tokens, name: &str) -> Result<(), ParseError> {
        self.output.push(name.to_string());
        self.eat(token)
    }

    pub fn sense(&mut self) -> Result<(), ParseError> {
        if let Some(tok) = self.current_token() {
            match tok {
                Tokens::RED => {
                self.handle_token(Tokens::RED, "red")?;
                print!("{}", "██".red());
                Ok(())
                }
                Tokens::GREEN => {
                    self.handle_token(Tokens::GREEN, "green")?;
                    print!("{}", "██".green());
                    Ok(())
                }
                Tokens::BLUE => {
                    self.handle_token(Tokens::BLUE, "blue")?;
                    print!("{}", "██".blue());
                    Ok(())
                }
                Tokens::NEWLINE => {
                    self.handle_token(Tokens::NEWLINE, "\n")?;
                    print!("\n");
                    Ok(())
                }
                
                Tokens::REPEAT => {
                    self.eat(Tokens::REPEAT)?;

                    self.eat(Tokens::OPENBRACKET)?;

                    let repeat_count = if let Some(Tokens::VALUE(n)) = self.current_token() {
                        *n
                    } else {
                        return Err(ParseError::UnexpectedToken {
                            expected: Tokens::VALUE(0),
                            found: self.current_token().cloned(),
                        });
                    };
                    self.pos += 1;
                    self.eat(Tokens::CLOSEBRACKET)?;
                    let mut last_line = Vec::new();
                    for t in self.output.iter().rev() {
                        if t == "\n" { break; }
                        last_line.push(t.clone());
                    }
                    last_line.reverse();

                    // Repeat line
                    for _ in 0..repeat_count {
                        for item in &last_line {
                            self.output.push(item.clone());
                            print!("{}", match item.as_str() {
                                "red" => "██".red(),
                                "green" => "██".green(),
                                "blue" => "██".blue(),
                                _ => "  ".normal(),
                            });
                        }
                        
                    }
                    Ok(())
                }
                Tokens::VALUE(_) => {
                
                    return Err(ParseError::UnexpectedToken {
                        expected: Tokens::REPEAT,
                        found: Some(tok.clone()),
                    });
                }
                Tokens::OPENBRACKET | Tokens::CLOSEBRACKET => {
                    // Brackets outside REPEAT
                    return Err(ParseError::UnexpectedToken {
                        expected: Tokens::REPEAT,
                        found: Some(tok.clone()),
                    });
                }
                _ => {
                    let name = format!("{:?}", tok);
                    self.output.push("unknown".to_string());
                    Err(ParseError::UnkownType { type_name: name })
                }
            }
        }
        else {
            Ok(()) 
        }

    }
    
}