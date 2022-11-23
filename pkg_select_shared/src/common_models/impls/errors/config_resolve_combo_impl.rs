use std::error::Error;
use crate::common_models::models::errors::config_resolve_combo::SerdeIODirectoryError;
use std::fmt;
use std::fmt::Formatter;

impl fmt::Display for SerdeIODirectoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            SerdeIODirectoryError::DirectoryResolveError(value) => value.fmt(f),
            SerdeIODirectoryError::SerdeError(value) => value.fmt(f),
            SerdeIODirectoryError::IOError(value) => value.fmt(f),
        }
    }
}

impl Error for SerdeIODirectoryError {}