use crate::common_models::models::configurations::target_binder::TargetBinder;
use std::path::PathBuf;

pub trait BinderSearch {
    fn find_binder_by_path<'t>(&'t self, process_path: &'t PathBuf) -> Option<&'t TargetBinder>;
}
