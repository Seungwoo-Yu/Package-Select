use struct_indexer_macro::{Indexed, ToAnyTrait};

pub struct RunnablePackage {}

#[derive(Clone, Indexed, ToAnyTrait)]
pub struct RunnablePackageUpdate {}

impl Default for RunnablePackageUpdate {
    fn default() -> Self {
        RunnablePackageUpdate {}
    }
}

#[derive(Clone, Indexed, ToAnyTrait)]
pub struct RunnablePackageDelete {}

impl Default for RunnablePackageDelete {
    fn default() -> Self {
        RunnablePackageDelete {}
    }
}