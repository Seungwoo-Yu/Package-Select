use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use crate::models::errors::path_binder_registration::{PathBinderRegistrationError, Type};

impl fmt::Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Type::BinderNotRegistered(value) => {
                write!(f, "Binder {} is not registered.", value)
            }
            Type::PathNotRegistered(value) => {
                write!(f, "Path of binder {} is not registered.", value)
            }
        }
    }
}

impl fmt::Display for PathBinderRegistrationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.error_type.fmt(f)
    }
}

impl Error for PathBinderRegistrationError {}
