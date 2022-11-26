use crate::common_models::models::errors::config_resolve_combo::SerdeIODirectoryError;
use crate::common_models::models::runtime_config::RuntimeConfig;

pub trait ConfigPersistence {
    fn exists(&self) -> Result<bool, SerdeIODirectoryError>;
    fn resolve(&self) -> Result<RuntimeConfig, SerdeIODirectoryError>;
    fn update(&self, config: &RuntimeConfig) -> Result<(), SerdeIODirectoryError>;
    fn reset(&self) -> Result<RuntimeConfig, SerdeIODirectoryError>;
}
