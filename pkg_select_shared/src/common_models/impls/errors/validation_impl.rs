use crate::common_models::models::errors::validation::{Type, ValidationError};
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

impl fmt::Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Validation Error: {}", self.error_type)
    }
}

impl Error for ValidationError {}
