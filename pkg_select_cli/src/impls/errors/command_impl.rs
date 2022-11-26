use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use crate::models::errors::command::CommandError;

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::None => Ok(()),
            CommandError::String(value) => {
                write!(f, "{}", value)
            }
            CommandError::Others(value) => {
                write!(f, "{}", value)
            }
        }
    }
}

impl Error for CommandError {}
