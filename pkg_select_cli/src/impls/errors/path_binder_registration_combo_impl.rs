use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use crate::models::errors::path_binder_registration_combo::DirectoryIOPathBinderRegistrationError;

impl fmt::Display for DirectoryIOPathBinderRegistrationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            DirectoryIOPathBinderRegistrationError::DirectoryResolveError(value) => {
                value.fmt(f)
            },
            DirectoryIOPathBinderRegistrationError::IOError(value) => {
                value.fmt(f)
            },
            DirectoryIOPathBinderRegistrationError::PathRegistrationError(value) => {
                value.fmt(f)
            },
            DirectoryIOPathBinderRegistrationError::WindowsError(value) => {
                value.fmt(f)
            },
            DirectoryIOPathBinderRegistrationError::FromUtf8Error(value) => {
                value.fmt(f)
            },
            DirectoryIOPathBinderRegistrationError::BinderRegistrationError(value) => {
                value.fmt(f)
            }
            DirectoryIOPathBinderRegistrationError::PathBinderRegistrationError(value) => {
                value.fmt(f)
            }
            DirectoryIOPathBinderRegistrationError::IOParseAlternativeResolveError(value) => {
                value.fmt(f)
            }
        }
    }
}

impl Error for DirectoryIOPathBinderRegistrationError {}