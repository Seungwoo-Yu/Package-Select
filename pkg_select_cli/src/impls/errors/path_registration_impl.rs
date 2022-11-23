use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use crate::models::errors::path_registration::{PathRegistrationError, Type};

impl fmt::Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Type::DuplicatedTarget(value) => {
                write!(f, "target {} is duplicated.", value)
            }
            Type::DuplicatedTargets => {
                write!(f, "multiple targets are duplicated.")
            }
            Type::DestinationNotFile(value) => {
                write!(f, "destination {} is not a file.", value)
            }
            Type::LinuxLinkGroupNotFound(value) => {
                write!(f, "link group {} is not found.", value)
            }
            Type::LinuxLinkItemNotFound(value) => {
                write!(f, "link item {} is not found.", value)
            }
        }
    }
}

impl fmt::Display for PathRegistrationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.error_type.fmt(f)
    }
}

impl Error for PathRegistrationError {}