#![forbid(unsafe_code)]
#![doc = "Typed foundations for the mentored key-value systems project."]

mod command;
mod key;

pub use command::{Command, CommandName, ParseError, parse_command};
pub use key::{Key, KeyError, MAX_KEY_BYTES};
