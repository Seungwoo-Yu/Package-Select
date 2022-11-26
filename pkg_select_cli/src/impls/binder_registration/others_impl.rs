#![cfg(not(target_os = "linux"))]

use std::{fs, io};
use std::cmp::Ordering;
use std::fs::{create_dir_all, File, remove_file};
use std::io::{BufReader, ErrorKind, Read};
use std::path::PathBuf;
use crate::models::binder_registration_resolver::BinderRegistrationResolver;
use crate::models::errors::binder_registration::BinderRegistrationError;
use crate::models::errors::binder_registration::Type::RunnerNotFound;
use crate::models::errors::binder_registration_combo::IOBinderRegistrationError;
use crate::traits::binder_registration::BinderRegistration;

impl BinderRegistration for BinderRegistrationResolver {
    fn registered(&self, exec_path: &PathBuf, process_path_without_filename: &PathBuf, source_name: &String) -> Result<bool, IOBinderRegistrationError> {
        if !exec_path.exists() {
            return Ok(false);
        }

        let runner_path = process_path_without_filename.join(source_name);

        match match_files(&runner_path, &exec_path) {
            Ok(value) => Ok(value),
            Err(error) => Err(IOBinderRegistrationError::IOError(error)),
        }
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

        match fs::copy(&runner_path, &exec_path) {
            Ok(_) => {},
            Err(error) => {
                return Err(IOBinderRegistrationError::IOError(error));
            }
        }

        if !cfg!(target_os = "windows") {
            let permission = match get_permission(&runner_path) {
                Ok(value) => value,
                Err(error) => {
                    println!("couldn't get permission from {}.", &exec_path.to_string_lossy());
                    return Err(IOBinderRegistrationError::IOError(error));
                }
            };

            if permission != 0o755 {
                match set_permission(&runner_path, 0o755) {
                    Ok(_) => {},
                    Err(error) => {
                        println!("couldn't set permission for {}.", &exec_path.to_string_lossy());
                        return Err(IOBinderRegistrationError::IOError(error));
                    }
                }
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

#[cfg(target_family = "unix")]
fn set_permission(target: &PathBuf, permission: u32) -> Result<(), io::Error> {
    use std::fs::Permissions;
    use std::os::unix::fs::PermissionsExt;

    fs::set_permissions(target, Permissions::from_mode(permission))
}

#[cfg(not(target_family = "unix"))]
fn set_permission(_: &PathBuf, _: u32) -> Result<(), io::Error> {
    return Ok(());
}

#[cfg(target_family = "unix")]
fn get_permission(target: &PathBuf) -> Result<u32, io::Error> {
    use std::fs::Permissions;
    use std::os::unix::fs::PermissionsExt;

    let metadata = match target.metadata() {
        Ok(value) => value,
        Err(error) => {
            return Err(error);
        }
    };

    Ok(metadata.permissions().mode())
}

#[cfg(not(target_family = "unix"))]
fn get_permission(_: &PathBuf) -> Result<u32, io::Error> {
    return Ok(1000u32);
}

fn match_files(target1: &PathBuf, target2: &PathBuf) -> Result<bool, io::Error> {
    let mut buffer1 = [0u8; 4096];
    let mut buffer2 = [0u8; 4096];

    let file1 = File::open(target1)?;
    let file2 = File::open(target2)?;

    let mut reader1 = BufReader::new(file1);
    let mut reader2 = BufReader::new(file2);

    loop {
        let read_operation1 = reader1.read_exact(&mut buffer1);
        let read_operation2 = reader2.read_exact(&mut buffer2);

        let eof_error_for_reader1 = match &read_operation1 {
            Ok(_) => false,
            Err(error) => error.kind() == ErrorKind::UnexpectedEof,
        };
        let eof_error_for_reader2 = match &read_operation2 {
            Ok(_) => false,
            Err(error) => error.kind() == ErrorKind::UnexpectedEof,
        };

        if eof_error_for_reader1 || eof_error_for_reader2 {
            return Ok(eof_error_for_reader1 == eof_error_for_reader2);
        }

        let verified = (&buffer1).iter()
            .zip(&buffer2)
            .map(| (x, y) | x.cmp(y))
            .find(| ord | *ord != Ordering::Equal)
            .is_none();

        if !verified {
            return Ok(false)
        }
    }
}
