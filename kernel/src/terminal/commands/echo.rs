use crate::println;
use crate::terminal::error::CliError;

pub fn echo(text: &str) -> Result<(), CliError> {
    println!("{text}");
    Ok(())
}