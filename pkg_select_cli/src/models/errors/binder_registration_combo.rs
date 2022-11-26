use std::io;
use crate::models::errors::binder_registration::BinderRegistrationError;
use crate::models::errors::path_binder_registration_combo::DirectoryIOPathBinderRegistrationError;

#[derive(Debug)]
pub enum IOBinderRegistrationError {
    IOError(io::Error),
    BinderRegistrationError(BinderRegistrationError),
}

impl IOBinderRegistrationError {
    pub fn to_path_binder_registration_combo(self) -> DirectoryIOPathBinderRegistrationError {
        match self {
            IOBinderRegistrationError::IOError(value) => {
                DirectoryIOPathBinderRegistrationError::IOError(value)
            }
            IOBinderRegistrationError::BinderRegistrationError(value) => {
                DirectoryIOPathBinderRegistrationError::BinderRegistrationError(value)
            }
        }
    }
}
