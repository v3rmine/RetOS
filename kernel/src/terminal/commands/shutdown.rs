use goolog::set_target;
use crate::println;
use crate::terminal::error::CliError;

pub fn shutdown() -> Result<(), CliError> {
    set_target!("SHUTDOWN");

    println!("shutting down");
    Ok(())
}