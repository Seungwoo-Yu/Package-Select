use std::error::Error;
use crate::common_models::models::errors::canonical_path_combo::IOCanonicalError;
use std::fmt;
use std::fmt::Formatter;

impl fmt::Display for IOCanonicalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            IOCanonicalError::IOError(value) => value.fmt(f),
            IOCanonicalError::CanonicalError(value) => value.fmt(f),
        }
    }
}

impl Error for IOCanonicalError {}
