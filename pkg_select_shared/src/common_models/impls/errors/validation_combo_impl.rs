use std::error::Error;
use crate::common_models::models::errors::validation_combo::IOCanonicalSerdeValidationError;
use std::fmt;
use std::fmt::Formatter;

impl fmt::Display for IOCanonicalSerdeValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            IOCanonicalSerdeValidationError::SerdeError(value) => value.fmt(f),
            IOCanonicalSerdeValidationError::ValidationError(value) => value.fmt(f),
            IOCanonicalSerdeValidationError::IOCanonicalError(value) => value.fmt(f),
        }
    }
}

impl Error for IOCanonicalSerdeValidationError {}
