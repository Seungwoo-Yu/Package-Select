use std::io;
use std::string::FromUtf8Error;
use linux_alternative_resolver_shared::common_models::models::errors::error_combo::IOParseAlternativeResolveError;
use pkg_select_shared::common_models::models::errors::directory_resolve::DirectoryResolveError;
use crate::models::errors::binder_registration::BinderRegistrationError;
use crate::models::errors::binder_registration_combo::IOBinderRegistrationError;
use crate::models::errors::path_binder_registration::PathBinderRegistrationError;
use crate::models::errors::path_registration::PathRegistrationError;
use crate::models::errors::path_registration_combo::DirectoryIOPathRegistrationError;
use crate::models::errors::windows_error_wrapper::WindowsError;

#[derive(Debug)]
pub enum DirectoryIOPathBinderRegistrationError {
    IOError(io::Error),
    BinderRegistrationError(BinderRegistrationError),
    DirectoryResolveError(DirectoryResolveError),
    PathRegistrationError(PathRegistrationError),
    WindowsError(WindowsError),
    FromUtf8Error(FromUtf8Error),
    PathBinderRegistrationError(PathBinderRegistrationError),
    IOParseAlternativeResolveError(IOParseAlternativeResolveError),
}

impl From<IOBinderRegistrationError> for DirectoryIOPathBinderRegistrationError {
    fn from(value: IOBinderRegistrationError) -> Self {
        value.to_path_binder_registration_combo()
    }
}

impl From<DirectoryIOPathRegistrationError> for DirectoryIOPathBinderRegistrationError {
    fn from(value: DirectoryIOPathRegistrationError) -> Self {
        value.to_path_binder_registration_combo()
    }
}
