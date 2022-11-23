#![cfg(all(not(target_os = "windows"), not(target_os = "linux")))]

use std::{env, fs};
use std::path::PathBuf;
use pkg_select_shared::user_dirs;
use crate::models::errors::path_registration::PathRegistrationError;
use crate::models::errors::path_registration::Type::{DuplicatedTarget, DuplicatedTargets};
use crate::models::errors::path_registration_combo::DirectoryIOPathRegistrationError;
use crate::models::path_registration_resolver::PathRegistrationResolver;
use crate::traits::path_registration::{MultiplePathRegistration, PathRegistration, PathRegistrationReset};

impl PathRegistration for PathRegistrationResolver {
    fn registered(&self, target: &PathBuf) -> Result<bool, DirectoryIOPathRegistrationError> {
        let mut path = PathBuf::new().join(target);

        if path.is_file() {
            let _ = &path.pop();
        }

        let path_string = path.to_string_lossy().to_string();

        Ok(append_env(
            &user_dir_path()?.join(PACKAGE_SELECT_ENV),
            &path_string,
            None,
            false,
        ).is_err())
    }

    fn register(&self, target: &PathBuf) -> Result<(), DirectoryIOPathRegistrationError> {
        let mut path = PathBuf::new().join(target);

        if path.is_file() {
            let _ = &path.pop();
        }

        let path_string = path.to_string_lossy().to_string();
        let env_path = user_dir_path()?.join(PACKAGE_SELECT_ENV);

        update_envs(
            &env_path,
            append_env(&env_path, &path_string, None, false)?
        )?;
        register_to_profiles()?;

        Ok(())
    }

    fn unregister(&self, target: &PathBuf) -> Result<(), DirectoryIOPathRegistrationError> {
        let mut path = PathBuf::new().join(target);

        if path.is_file() {
            let _ = &path.pop();
        }

        let path_string = path.to_string_lossy().to_string();
        let env_path = user_dir_path()?.join(PACKAGE_SELECT_ENV);

        update_envs(
            &env_path,
            remove_env(&env_path, &path_string, None)?
        )?;

        Ok(())
    }
}

impl MultiplePathRegistration for PathRegistrationResolver {
    fn register(&self, target: &Vec<&PathBuf>) -> Result<(), DirectoryIOPathRegistrationError> {
        let paths: Vec<String> = target.iter().map(| value | {
            let mut path = PathBuf::new().join(value);

            if path.is_file() {
                let _ = &path.pop();
            }

            path.to_string_lossy().to_string()
        }).collect();
        let env_path = user_dir_path()?.join(PACKAGE_SELECT_ENV);

        update_envs(
            &env_path,
            append_envs(&env_path, &paths, None, false)?
        )?;
        register_to_profiles()?;

        Ok(())
    }

    fn unregister(&self, target: &Vec<&PathBuf>) -> Result<(), DirectoryIOPathRegistrationError> {
        let paths: Vec<String> = target.iter().map(| value | {
            let mut path = PathBuf::new().join(value);

            if path.is_file() {
                let _ = &path.pop();
            }

            path.to_string_lossy().to_string()
        }).collect();
        let env_path = user_dir_path()?.join(PACKAGE_SELECT_ENV);

        update_envs(
            &env_path,
            remove_envs(&env_path, &paths, None)?
        )?;

        Ok(())
    }
}

impl PathRegistrationReset for PathRegistrationResolver {
    fn reset(&self) -> Result<(), DirectoryIOPathRegistrationError> {
        reset()
    }
}

const PACKAGE_SELECT_ENV: &str = ".package-select-env";
const PROFILES: [&str; 4] = [
    ".zshenv", // usually for decent macOS
    ".profile", // for macOS and Linux
    ".bash_profile", // for Linux and old macOS
    ".bashrc", // for Linux and old macOS
];

fn format_raw_data(raw_data: &String) -> String {
    format!("export PATH=\"$PATH:{}\"", raw_data)
}

fn format_path(path: &PathBuf) -> String {
    format!("source {}", (path).to_string_lossy())
}

fn user_dir_path() -> Result<PathBuf, DirectoryIOPathRegistrationError> {
    if cfg!(target_os = "linux") {
        match env::vars().find(| (key, _) | {
            key.to_lowercase().eq("sudo_user")
        }) {
            None => {},
            Some((_, value)) => {
                return Ok(PathBuf::from(&format!("/home/{}", value)));
            }
        }
    }

    let user_dir = match user_dirs() {
        Ok(value) => value,
        Err(error) => {
            return Err(DirectoryIOPathRegistrationError::DirectoryResolveError(error));
        }
    };

    Ok(user_dir.home_dir().to_path_buf())
}

fn append_env(
    path: &PathBuf,
    env: &String,
    overridden_contents: Option<String>,
    ignore_duplication: bool,
) -> Result<Vec<String>, DirectoryIOPathRegistrationError> {
    let formatted_env = format_raw_data(env);
    let file = overridden_contents.unwrap_or(
        match fs::read_to_string(path) {
            Ok(value) => value,
            Err(_) => format!("")
        }
    );
    let mut file_modified: Vec<&str> = file.lines().collect();

    file_modified.retain(|value | !value.contains(&formatted_env));

    if !ignore_duplication && file.lines().count() != file_modified.len() {
        return Err(
            DirectoryIOPathRegistrationError::PathRegistrationError(
                PathRegistrationError { error_type: DuplicatedTarget(env.to_string()) }
            )
        );
    }

    file_modified.push(&formatted_env);

    Ok(file_modified.iter().map(| value | value.to_string()).collect())
}

fn append_envs(
    path: &PathBuf,
    env: &Vec<String>,
    overridden_contents: Option<String>,
    ignore_duplication: bool,
) -> Result<Vec<String>, DirectoryIOPathRegistrationError> {
    let mut formatted_envs: Vec<String> = env.iter()
        .map(| value | format_raw_data(value))
        .collect();
    let file = overridden_contents.unwrap_or(
        match fs::read_to_string(path) {
            Ok(value) => value,
            Err(_) => format!("")
        }
    );
    let mut file_modified: Vec<&str> = file.lines().collect();

    for value in file_modified.iter() {
        if formatted_envs.len() == 0 { break; }

        let search_position = formatted_envs.iter()
            .position(| value2 | value.contains(value2));

        match search_position {
            None => {}
            Some(value) => {
                formatted_envs.remove(value);
            }
        }
    }

    if !ignore_duplication && env.len() != formatted_envs.len() {
        return Err(
            DirectoryIOPathRegistrationError::PathRegistrationError(
                PathRegistrationError { error_type: DuplicatedTargets }
            )
        );
    }

    for value in formatted_envs.iter() {
        file_modified.push(value);
    }

    Ok(file_modified.iter().map(| value | value.to_string()).collect())
}

fn remove_env(
    path: &PathBuf,
    env: &String,
    overridden_contents: Option<String>,
) -> Result<Vec<String>, DirectoryIOPathRegistrationError> {
    let formatted_env = format_raw_data(env);
    let file = overridden_contents.unwrap_or(
        match fs::read_to_string(path) {
            Ok(value) => value,
            Err(_) => format!("")
        }
    );
    let mut file_modified: Vec<&str> = file.lines().collect();

    file_modified.retain(|value | !value.contains(&formatted_env));

    Ok(file_modified.iter().map(| value | value.to_string()).collect())
}

fn remove_envs(
    path: &PathBuf,
    env: &Vec<String>,
    overridden_contents: Option<String>,
) -> Result<Vec<String>, DirectoryIOPathRegistrationError> {
    let mut formatted_envs: Vec<String> = env.iter()
        .map(| value | format_raw_data(value))
        .collect();
    let file = overridden_contents.unwrap_or(
        match fs::read_to_string(path) {
            Ok(value) => value,
            Err(_) => format!("")
        }
    );
    let file_modified: Vec<&str> = file.lines().collect();
    let mut file_output: Vec<String> = vec![];

    for value in file_modified.iter() {
        if formatted_envs.len() == 0 {
            file_output.push(value.to_string());
            break;
        }

        let search_position = formatted_envs.iter()
            .position(| value2 | value.contains(value2));

        match search_position {
            None => {
                file_output.push(value.to_string());
            }
            Some(value) => {
                formatted_envs.remove(value);
            }
        }
    }

    Ok(file_output)
}

fn update_envs(path: &PathBuf, envs: Vec<String>) -> Result<(), DirectoryIOPathRegistrationError> {
    match fs::write(path, envs.join("\n")) {
        Ok(_) => {}
        Err(error) => {
            return Err(DirectoryIOPathRegistrationError::IOError(error));
        }
    };

    Ok(())
}

fn register_to_profiles() -> Result<(), DirectoryIOPathRegistrationError> {
    let user_dir = user_dir_path()?;
    let env_path = user_dir.join(PACKAGE_SELECT_ENV);
    let formatted_env_path = format_path(&env_path);

    'profile_iteration : for value in PROFILES.iter() {
        let profile_path = user_dir.join(value);

        if (&profile_path).exists() {
            let mut profile_file = match fs::read_to_string(&profile_path) {
                Ok(value) => value,
                Err(error) => {
                    return Err(DirectoryIOPathRegistrationError::IOError(error));
                }
            };

            for value in profile_file.lines() {
                if value.contains(&formatted_env_path) {
                    continue 'profile_iteration;
                }
            }

            match profile_file.lines().last() {
                None => {},
                Some(value) => {
                    if !value.eq("\n") {
                        profile_file.push('\n');
                    }
                }
            }

            profile_file.push_str(&format!("{}\n", &formatted_env_path));

            match fs::write(&profile_path, &profile_file) {
                Ok(_) => {},
                Err(error) => {
                    return Err(DirectoryIOPathRegistrationError::IOError(error));
                }
            }
        } else {
            match fs::write(&profile_path, &formatted_env_path) {
                Ok(_) => {},
                Err(error) => {
                    return Err(DirectoryIOPathRegistrationError::IOError(error));
                }
            }
        }
    }

    Ok(())
}

fn remove_from_profiles() -> Result<(), DirectoryIOPathRegistrationError> {
    let user_dir = user_dir_path()?;
    let env_path = user_dir.join(PACKAGE_SELECT_ENV);
    let formatted_env_path = format_path(&env_path);

    for value in PROFILES.iter() {
        let profile_path = user_dir.join(value);

        if (&profile_path).exists() {
            let profile_file = match fs::read_to_string(&profile_path) {
                Ok(value) => value,
                Err(error) => {
                    return Err(DirectoryIOPathRegistrationError::IOError(error));
                }
            };
            let mut profile_file_modified: Vec<&str> = profile_file.lines().collect();

            (&mut profile_file_modified).retain(| value | !value.contains(&formatted_env_path));

            match fs::write(&profile_path, profile_file_modified.join("\n")) {
                Ok(_) => {},
                Err(error) => {
                    return Err(DirectoryIOPathRegistrationError::IOError(error));
                }
            }
        }
    }

    Ok(())
}

fn reset() -> Result<(), DirectoryIOPathRegistrationError> {
    remove_from_profiles()?;
    remove_env_file(&user_dir_path()?.join(PACKAGE_SELECT_ENV))
}

fn remove_env_file(path: &PathBuf) -> Result<(), DirectoryIOPathRegistrationError> {
    match fs::remove_file(path) {
        Ok(_) => {}
        Err(error) => {
            return Err(DirectoryIOPathRegistrationError::IOError(error));
        }
    };

    Ok(())
}
