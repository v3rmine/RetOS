use goolog::set_target;
use crate::println;
use crate::terminal::error::CliError;

pub fn echo(text: &str) -> Result<(), CliError> {
    set_target!("ECHO");

    println!("{text}");
    Ok(())
}