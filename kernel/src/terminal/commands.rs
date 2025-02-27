use crate::printer::buffer::Writer;
use core::convert::Infallible;
use embedded_cli::cli::Cli;
use embedded_cli::Command;

#[derive(Command)]
enum Command<'a> {
    /// Echoes the following argument
    Echo {
        /// Text to echo
        text: &'a str,
    },

    /// Shutdown the operating system
    Shutdown,
}

pub fn handle_command(cli: &mut Cli<&mut Writer, Infallible, [u8; 100], [u8; 100]>, byte: u8) {
    use core::fmt::Write;
    
    cli.process_byte::<Command<'_>, _>(
        byte,
        &mut Command::processor(|cli, command| match command {
            Command::Echo { text } => {
                write!(cli.writer(), "{}", text).unwrap();
                Ok(())
            }
            Command::Shutdown => {
                write!(cli.writer(), "shutting down").unwrap();
                Ok(())
            }
        }),
    )
        .expect("CLI processing failed");
}