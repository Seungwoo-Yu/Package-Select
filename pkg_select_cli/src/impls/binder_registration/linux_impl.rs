#![cfg(target_os = "linux")]

use std::fs::{create_dir_all, remove_file};
use std::os::unix::fs::symlink;
use std::path::PathBuf;
use crate::models::binder_registration_resolver::BinderRegistrationResolver;
use crate::models::errors::binder_registration::BinderRegistrationError;
use crate::models::errors::binder_registration::Type::RunnerNotFound;
use crate::models::errors::binder_registration_combo::IOBinderRegistrationError;
use crate::traits::binder_registration::BinderRegistration;

impl BinderRegistration for BinderRegistrationResolver {
    fn registered(&self, exec_path: &PathBuf, process_path_without_filename: &PathBuf, source_name: &String) -> Result<bool, IOBinderRegistrationError> {
        let runner_path = process_path_without_filename.join(source_name);

        if !(&exec_path).exists() || !(&runner_path).exists() || !(&exec_path).is_symlink() {
            return Ok(false);
        }

        let link_path = match (&exec_path).read_link() {
            Ok(value) => value,
            Err(error) => {
                return Err(IOBinderRegistrationError::IOError(error));
            }
        };

        Ok((&link_path).eq(&runner_path))
    }

    fn register(&self, exec_path: &PathBuf, process_path_without_filename: &PathBuf, source_name: &String) -> Result<(), IOBinderRegistrationError> {
        if self.registered(exec_path, process_path_without_filename, source_name)? {
            println!("{} already exists. Skipping to create it...", &exec_path.to_string_lossy());
            return Ok(());
        }

        let runner_path = process_path_without_filename.join(source_name);

        if !(&runner_path).exists() {
            println!("couldn't find Package Select Runner.");
            println!("path: {}", &runner_path.to_string_lossy());
            return Err(
                IOBinderRegistrationError::BinderRegistrationError(
                    BinderRegistrationError {
                        error_type: RunnerNotFound((&runner_path).to_string_lossy().to_string())
                    }
                )
            )
        }

        match exec_path.parent() {
            None => {}
            Some(value) => {
                match create_dir_all(value) {
                    Ok(_) => {}
                    Err(error) => {
                        return Err(IOBinderRegistrationError::IOError(error));
                    }
                }
            }
        }

        if (&exec_path).exists() {
            println!("{} already exists, but it's not proper runner file. Removing old file...", &exec_path.to_string_lossy());
            match remove_file(&exec_path) {
                Ok(_) => {}
                Err(error) => {
                    return Err(IOBinderRegistrationError::IOError(error));
                }
            }
        }

        match symlink(&runner_path, &exec_path) {
            Ok(_) => {},
            Err(error) => {
                return Err(IOBinderRegistrationError::IOError(error));
            }
        }

        println!("Runner {} is created.", &exec_path.to_string_lossy());

        Ok(())
    }

    fn unregister(&self, exec_path: &PathBuf) -> Result<(), IOBinderRegistrationError> {
        if (&exec_path).exists() {
            match remove_file(&exec_path) {
                Ok(_) => {}
                Err(error) => {
                    return Err(IOBinderRegistrationError::IOError(error));
                }
            }

            println!("Runner {} is removed.", &exec_path.to_string_lossy());
        } else {
            println!("{} is already removed. Skipping to remove it...", &exec_path.to_string_lossy());
        }

        Ok(())
    }
}
