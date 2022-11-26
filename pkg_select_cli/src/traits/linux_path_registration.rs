use std::path::PathBuf;
use pkg_select_shared::common_models::models::runtime_config::RuntimeConfig;
use crate::models::errors::path_registration_combo::DirectoryIOPathRegistrationError;

pub trait LinuxPathRegistration {
    fn registered(&self, target: &PathBuf) -> Result<bool, DirectoryIOPathRegistrationError>;
    fn register(&mut self, target: &PathBuf) -> Result<(), DirectoryIOPathRegistrationError>;
    fn unregister(&mut self, target: &PathBuf) -> Result<(), DirectoryIOPathRegistrationError>;
}

pub trait MultipleLinuxPathRegistration {
    fn register(&mut self, target: &Vec<&PathBuf>) -> Result<(), DirectoryIOPathRegistrationError>;
    fn unregister(&mut self, target: &Vec<&PathBuf>) -> Result<(), DirectoryIOPathRegistrationError>;
}

pub trait LinuxPathRegistrationReset {
    fn reset(&mut self, config: &RuntimeConfig) -> Result<(), DirectoryIOPathRegistrationError>;
}
