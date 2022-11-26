use crate::common_models::models::errors::config_resolve_combo::SerdeIODirectoryError;
use std::path::PathBuf;

pub trait ConfigPath {
    fn project_dir(&self) -> Result<PathBuf, SerdeIODirectoryError>;
    fn config_file_name(&self) -> String {
        "config.json".to_string()
    }
}
