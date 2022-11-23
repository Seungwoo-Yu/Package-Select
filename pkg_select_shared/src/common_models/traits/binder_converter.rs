use std::path::PathBuf;

pub trait BinderConverter {
    fn convert_target_to_pathbuf(&self) -> PathBuf;
    fn convert_exec_to_pathbuf(&self) -> PathBuf;
}
