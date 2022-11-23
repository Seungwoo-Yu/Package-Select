use std::io;
use std::string::FromUtf8Error;
use linux_alternative_resolver_shared::common_models::models::errors::error_combo::IOParseAlternativeResolveError;
use pkg_select_shared::common_models::models::errors::directory_resolve::DirectoryResolveError;
use crate::models::errors::path_binder_registration_combo::DirectoryIOPathBinderRegistrationError;
use crate::models::errors::path_registration::PathRegistrationError;
use crate::models::errors::windows_error_wrapper::WindowsError;

#[derive(Debug)]
pub enum DirectoryIOPathRegistrationError {
    IOError(io::Error),
    DirectoryResolveError(DirectoryResolveError),
    PathRegistrationError(PathRegistrationError),
    WindowsError(WindowsError),
    FromUtf8Error(FromUtf8Error),
    IOParseAlternativeResolveError(IOParseAlternativeResolveError),
}

impl DirectoryIOPathRegistrationError {
    pub fn to_path_binder_registration_combo(self) -> DirectoryIOPathBinderRegistrationError {
        match self {
            DirectoryIOPathRegistrationError::IOError(value) => {
                DirectoryIOPathBinderRegistrationError::IOError(value)
            }
            DirectoryIOPathRegistrationError::DirectoryResolveError(value) => {
                DirectoryIOPathBinderRegistrationError::DirectoryResolveError(value)
            }
            DirectoryIOPathRegistrationError::PathRegistrationError(value) => {
                DirectoryIOPathBinderRegistrationError::PathRegistrationError(value)
            }
            DirectoryIOPathRegistrationError::WindowsError(value) => {
                DirectoryIOPathBinderRegistrationError::WindowsError(value)
            }
            DirectoryIOPathRegistrationError::FromUtf8Error(value) => {
                DirectoryIOPathBinderRegistrationError::FromUtf8Error(value)
            }
            DirectoryIOPathRegistrationError::IOParseAlternativeResolveError(value) => {
                DirectoryIOPathBinderRegistrationError::IOParseAlternativeResolveError(value)
            }
        }
    }
}