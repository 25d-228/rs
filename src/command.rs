use std::error::Error;
use std::fmt;

use crate::{Key, KeyError};

/// The recognized command name, retained in arity errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandName {
    Get,
    Delete,
}

/// A parsed command with validated, owned data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Get(Key),
    Delete(Key),
}

/// A typed command-parsing failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    EmptyInput,
    UnknownCommand {
        command: String,
    },
    MissingKey {
        command: CommandName,
    },
    UnexpectedArgument {
        command: CommandName,
        argument: String,
    },
    InvalidKey(KeyError),
}

/// Parses one `GET <key>` or `DELETE <key>` command.
pub fn parse_command(_input: &str) -> Result<Command, ParseError> {
    todo!("R01: implement command parsing")
}

impl fmt::Display for ParseError {
    fn fmt(&self, _formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("R01: implement ParseError formatting")
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        todo!("R01: expose the nested KeyError source when present")
    }
}
