use crate::common_models::models::errors::config_resolve_combo::SerdeIODirectoryError;
use crate::config_resolver::category_resolver::CategoryResolver;
use crate::config_resolver::package_resolver::PackageResolver;
use crate::config_resolver::traits::config_path::ConfigPath;
use std::path::PathBuf;

pub struct ConfigResolver {
    pub category_resolver: CategoryResolver,
    pub package_resolver: PackageResolver,
}

impl Default for ConfigResolver {
    fn default() -> Self {
        ConfigResolver {
            category_resolver: CategoryResolver {},
            package_resolver: PackageResolver {},
        }
    }
}

impl ConfigResolver {
    pub(crate) fn config_path(&self) -> Result<PathBuf, SerdeIODirectoryError> {
        let project_dir = match self.project_dir() {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };

        Ok(PathBuf::new()
            .join(project_dir)
            .join(self.config_file_name()))
    }
}
