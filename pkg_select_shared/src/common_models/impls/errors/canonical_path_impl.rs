use crate::common_models::models::errors::canonical_path::CanonicalPathError;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

impl fmt::Display for CanonicalPathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Canonical Path Conversion Error")
    }
}

impl Error for CanonicalPathError {}
