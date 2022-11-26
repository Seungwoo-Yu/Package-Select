use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use crate::models::errors::windows_error_wrapper::{ErrorCodes, WindowsError};

impl fmt::Display for ErrorCodes {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for WindowsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let message = &self.message;
        if message.is_empty() {
            write!(f, "{}", &self.code)
        } else {
            write!(f, "{} ({})", message, &self.code)
        }
    }
}

impl Error for WindowsError {}
