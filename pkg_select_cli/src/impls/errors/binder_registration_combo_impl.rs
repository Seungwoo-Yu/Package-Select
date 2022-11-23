use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use crate::models::errors::binder_registration_combo::IOBinderRegistrationError;

impl fmt::Display for IOBinderRegistrationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            IOBinderRegistrationError::IOError(value) => {
                value.fmt(f)
            }
            IOBinderRegistrationError::BinderRegistrationError(value) => {
                value.fmt(f)
            }
        }
    }
}

impl Error for IOBinderRegistrationError {}