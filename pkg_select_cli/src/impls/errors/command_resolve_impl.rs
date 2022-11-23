use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use crate::models::errors::command_resolve::{CommandResolveError, Type};

impl fmt::Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Type::DuplicatedName(value) => {
                write!(f, "command name {} is duplicated.", value)
            }
            Type::EmptyNameList => {
                write!(f, "one of commands contains empty name list.")
            }
        }
    }
}

impl fmt::Display for CommandResolveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.error_type.fmt(f)
    }
}

impl Error for CommandResolveError {}