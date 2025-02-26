use crate::println;
use core::convert::Infallible;
use embedded_cli::cli::Cli;
use embedded_cli::writer::EmptyWriter;
use embedded_cli::Command;

#[derive(Command)]
enum Command<'a> {
    /// Echoes the following argument
    Echo {
        /// Text to echo
        text: Option<&'a str>,
    },

    /// Shutdown the router
    Shutdown,
}

pub fn handle_command(cli: &mut Cli<EmptyWriter, Infallible, [u8;100], [u8;100]>, byte: u8) {
    cli.process_byte::<Command<'_>, _>(
        byte,
        &mut Command::processor(|_cli, command| match command {
            Command::Echo { text } => {
                println!("{}", text.unwrap_or(""));
                Ok(())
            }
            Command::Shutdown => {
                println!("shutting down");
                // TODO implement shutdown
                Ok(())
            }
            _ => {
                println!("Unknown command");
                Ok(())
            }
        }),
    )
        .expect("CLI processing failed");
}