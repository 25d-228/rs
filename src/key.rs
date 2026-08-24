use std::error::Error;
use std::fmt;

/// Maximum accepted key length, measured in UTF-8 bytes.
pub const MAX_KEY_BYTES: usize = 64;

/// An owned key that has passed all validation rules.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Key(String);

impl Key {
    /// Validates an owned string and constructs a key.
    pub fn new(_raw: String) -> Result<Self, KeyError> {
        todo!("R01: implement key validation")
    }

    /// Borrows the validated key text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the key and returns its owned text.
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

/// A reason that input could not become a valid [`Key`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyError {
    Empty,
    TooLong { max: usize, actual: usize },
    ContainsWhitespace { byte_index: usize },
    ContainsControl { byte_index: usize },
}

impl fmt::Display for KeyError {
    fn fmt(&self, _formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("R01: implement KeyError formatting")
    }
}

impl Error for KeyError {}
