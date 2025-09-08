use std::io::{self, Write};

#[derive(Debug)]
pub enum CliError {
    ReadError(std::io::Error),
}

pub fn cli_ui() -> Result<String, CliError>
{
    print!("-> ");
    io::stdout().flush().map_err(|e| CliError::ReadError(e))?;
    
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .map_err(|e| CliError::ReadError(e))?;
    
    Ok(input.trim().to_string())
}