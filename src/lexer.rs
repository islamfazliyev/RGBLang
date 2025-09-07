use crate::tokens::{Tokens};

pub  fn lexer(content: &str) -> Result<Vec<Tokens>, String>
{
    let mut token: Vec<Tokens> = Vec::new();
    let mut chars = content.chars().peekable();
    let mut in_repeat = false;

    while let Some(c) = chars.next() {
        match c {
            '0' if !in_repeat => token.push(Tokens::RED),
            '1' if !in_repeat => token.push(Tokens::GREEN),
            '2' if !in_repeat => token.push(Tokens::BLUE),
            ',' => token.push(Tokens::NEWLINE),
            'R' => {
                token.push(Tokens::REPEAT);
                if let Some(&'(') = chars.peek() {
                    in_repeat = true;
                }},
            '(' => {
                token.push(Tokens::OPENBRACKET);
               },
            ')' => {
                token.push(Tokens::CLOSEBRACKET);
                in_repeat = false;
            },
            d if d.is_ascii_digit() && in_repeat => {
                let mut number = d.to_string();
                while let Some(peek) = chars.peek() {
                    if peek.is_ascii_digit() {
                        number.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                let value = number.parse::<i32>().map_err(|e| e.to_string())?;
                token.push(Tokens::VALUE(value));
            },

            d if d.is_ascii_digit() => {
                // Handle digits outside repeat context as colors
                match d {
                    '0' => token.push(Tokens::RED),
                    '1' => token.push(Tokens::GREEN),
                    '2' => token.push(Tokens::BLUE),
                    _ => return Err(format!("Unexpected digit: '{}'", d)),
                }
            },
            _ => {
                return Err(format!("Unexpected character: '{}'", c));
            },
        }
    }

    Ok(token)
}