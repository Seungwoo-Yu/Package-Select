use struct_indexer_macro::{Indexed, ToAnyTrait};

pub struct PackageCategory {}

#[derive(Clone, Indexed, ToAnyTrait)]
pub struct PackageCategoryUpdate {}

impl Default for PackageCategoryUpdate {
    fn default() -> Self {
        PackageCategoryUpdate {}
    }
}

#[derive(Clone, Indexed, ToAnyTrait)]
pub struct PackageCategoryDelete {}

impl Default for PackageCategoryDelete {
    fn default() -> Self {
        PackageCategoryDelete {}
    }
}
