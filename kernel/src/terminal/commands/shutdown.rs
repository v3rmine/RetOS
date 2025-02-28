use crate::println;
use crate::terminal::error::CliError;

pub fn shutdown() -> Result<(), CliError> {
    println!("shutting down");
    Ok(())
}