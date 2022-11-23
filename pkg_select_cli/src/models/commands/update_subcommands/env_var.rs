use struct_indexer_macro::{Indexed, ToAnyTrait};

pub struct EnvVar {}

#[derive(Clone, Indexed, ToAnyTrait)]
pub struct EnvVarUpdate {}

impl Default for EnvVarUpdate {
    fn default() -> Self {
        EnvVarUpdate {}
    }
}

#[derive(Clone, Indexed, ToAnyTrait)]
pub struct EnvVarDelete {}

impl Default for EnvVarDelete {
    fn default() -> Self {
        EnvVarDelete {}
    }
}
