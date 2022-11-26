use crate::common_models::models::errors::config_resolve_combo::SerdeIODirectoryError;
use crate::common_models::models::runtime_config::RuntimeConfig;
use crate::config_resolver::config_resolver::ConfigResolver;
use crate::config_resolver::traits::config_path::ConfigPath;
use crate::config_resolver::traits::config_persistence::ConfigPersistence;
use crate::{project_dirs, PathPop};
use std::{env, fs};
use std::path::PathBuf;

impl ConfigPersistence for ConfigResolver {
    fn exists(&self) -> Result<bool, SerdeIODirectoryError> {
        let _path = self.config_path();
        let path = match _path {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };

        Ok(path.exists())
    }

    fn resolve(&self) -> Result<RuntimeConfig, SerdeIODirectoryError> {
        match self.exists() {
            Ok(value) => {
                if !value {
                    match self.reset() {
                        Ok(_) => {}
                        Err(error) => return Err(error),
                    }
                }
            }
            Err(error) => return Err(error),
        }

        let path = match self.config_path() {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let raw_data = match fs::read_to_string(path) {
            Ok(value) => value,
            Err(error) => {
                return Err(SerdeIODirectoryError::IOError(error));
            }
        };

        match serde_json::from_str(&raw_data) {
            Ok(value) => Ok(value),
            Err(error) => {
                return Err(SerdeIODirectoryError::SerdeError(error));
            }
        }
    }

    fn update(&self, config: &RuntimeConfig) -> Result<(), SerdeIODirectoryError> {
        match self.exists() {
            Ok(_) => {}
            Err(_) => {
                return match self.reset() {
                    Ok(_) => Ok(()),
                    Err(error) => Err(error),
                }
            }
        }

        let raw_data = match serde_json::to_string_pretty(&config) {
            Ok(value) => value,
            Err(error) => {
                return Err(SerdeIODirectoryError::SerdeError(error));
            }
        };

        let path = match self.config_path() {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };
        let path_without_filename = path.pop_path();

        if !path_without_filename.exists() {
            match fs::create_dir_all(&path_without_filename) {
                Ok(_) => {}
                Err(error) => {
                    return Err(SerdeIODirectoryError::IOError(error));
                }
            }
        }

        dbg!(&path);
        dbg!(&raw_data);

        return match fs::write(&path, &raw_data) {
            Ok(_) => Ok(()),
            Err(error) => Err(SerdeIODirectoryError::IOError(error)),
        };
    }

    fn reset(&self) -> Result<RuntimeConfig, SerdeIODirectoryError> {
        let runtime_config = RuntimeConfig::default();

        return match self.update(&runtime_config) {
            Ok(_) => Ok(runtime_config),
            Err(error) => Err(error),
        };
    }
}

impl ConfigPath for ConfigResolver {
    fn project_dir(&self) -> Result<PathBuf, SerdeIODirectoryError> {
        if cfg!(target_os = "linux") {
            let vars: Vec<(String, String)> = env::vars().collect();

            for (key, value) in vars.iter() {
                if key.to_lowercase().eq("sudo_user") {
                    return Ok(PathBuf::from(format!("/home/{}/.config", value)));
                }
            }
        }

        let project_dir = match project_dirs() {
            Ok(value) => value,
            Err(error) => {
                return Err(SerdeIODirectoryError::DirectoryResolveError(error));
            }
        };

        return Ok(project_dir.config_dir().to_path_buf());
    }
}
