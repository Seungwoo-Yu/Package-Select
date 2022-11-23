use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use crate::models::errors::binder_registration::{BinderRegistrationError, Type};

impl fmt::Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Type::RunnerNotFound(value) => {
                write!(f, "runner {} is not found.", value)
            }
        }
    }
}

impl fmt::Display for BinderRegistrationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.error_type.fmt(f)
    }
}

impl Error for BinderRegistrationError {}