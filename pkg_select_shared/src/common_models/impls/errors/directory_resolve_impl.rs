use crate::common_models::models::errors::directory_resolve::{DirectoryResolveError, Type};
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

impl fmt::Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for DirectoryResolveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Directory Resolve Error: {}", self.error_type)
    }
}

impl Error for DirectoryResolveError {}
