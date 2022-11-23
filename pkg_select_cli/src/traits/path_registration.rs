use std::path::PathBuf;
use crate::models::errors::path_registration_combo::DirectoryIOPathRegistrationError;

pub trait PathRegistration {
    fn registered(&self, target: &PathBuf) -> Result<bool, DirectoryIOPathRegistrationError>;
    fn register(&self, target: &PathBuf) -> Result<(), DirectoryIOPathRegistrationError>;
    fn unregister(&self, target: &PathBuf) -> Result<(), DirectoryIOPathRegistrationError>;
}

pub trait MultiplePathRegistration {
    fn register(&self, target: &Vec<&PathBuf>) -> Result<(), DirectoryIOPathRegistrationError>;
    fn unregister(&self, target: &Vec<&PathBuf>) -> Result<(), DirectoryIOPathRegistrationError>;
}

pub trait PathRegistrationReset {
    fn reset(&self) -> Result<(), DirectoryIOPathRegistrationError>;
}