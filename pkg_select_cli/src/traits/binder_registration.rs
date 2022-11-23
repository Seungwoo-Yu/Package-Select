use std::path::PathBuf;
use crate::models::errors::binder_registration_combo::IOBinderRegistrationError;

pub trait BinderRegistration {
    fn registered(&self, exec_path: &PathBuf, process_path_without_filename: &PathBuf, source_name: &String) -> Result<bool, IOBinderRegistrationError>;
    fn register(&self, exec_path: &PathBuf, process_path_without_filename: &PathBuf, source_name: &String) -> Result<(), IOBinderRegistrationError>;
    fn unregister(&self, exec_path: &PathBuf) -> Result<(), IOBinderRegistrationError>;
}

pub trait BinderRegistrationReset {
    fn reset(&self) -> Result<(), IOBinderRegistrationError>;
}