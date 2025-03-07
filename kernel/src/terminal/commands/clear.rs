use goolog::set_target;
use crate::printer::buffer::WRITER;
use crate::terminal::error::CliError;

pub fn clear() -> Result<(), CliError> {
    set_target!("CLEAR");

    WRITER.write().clear();
    Ok(())
}