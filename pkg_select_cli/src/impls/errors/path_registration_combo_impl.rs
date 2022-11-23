use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use crate::models::errors::path_registration_combo::DirectoryIOPathRegistrationError;

impl fmt::Display for DirectoryIOPathRegistrationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            DirectoryIOPathRegistrationError::DirectoryResolveError(value) => {
                value.fmt(f)
            },
            DirectoryIOPathRegistrationError::IOError(value) => {
                value.fmt(f)
            }
            DirectoryIOPathRegistrationError::PathRegistrationError(value) => {
                value.fmt(f)
            }
            DirectoryIOPathRegistrationError::WindowsError(value) => {
                value.fmt(f)
            }
            DirectoryIOPathRegistrationError::FromUtf8Error(value) => {
                value.fmt(f)
            }
            DirectoryIOPathRegistrationError::IOParseAlternativeResolveError(value) => {
                value.fmt(f)
            }
        }
    }
}

impl Error for DirectoryIOPathRegistrationError {}