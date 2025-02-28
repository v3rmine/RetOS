use crate::terminal::commands::echo::echo;
use crate::terminal::commands::shutdown::shutdown;
use crate::terminal::error::CliError;
use crate::terminal::terminal::TerminalBuffer;
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

pub fn handle_command(cli: &mut Cli<&mut TerminalBuffer, CliError, [u8; 100], [u8; 100]>, byte: u8) {
    cli.process_byte::<Command<'_>, _>(
        byte,
        &mut Command::processor(|_cli, command| match command {
            Command::Echo { text } => echo(text),
            Command::Shutdown => shutdown()
        }),
    )
        .expect("CLI processing failed");
}