use crate::common_models::models::configurations::target_binder::TargetBinder;
use crate::common_models::traits::binder_converter::BinderConverter;
use std::path::PathBuf;

impl BinderConverter for TargetBinder {
    fn convert_target_to_pathbuf(&self) -> PathBuf {
        PathBuf::from(&self.target_path).join(&self.target_name)
    }

    fn convert_exec_to_pathbuf(&self) -> PathBuf {
        PathBuf::from(&self.execution_path).join(&self.target_name)
    }
}
