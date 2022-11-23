use struct_indexer_macro::{Indexed, ToAnyTrait};

pub struct TargetBinder {}

#[derive(Clone, Indexed, ToAnyTrait)]
pub struct TargetBinderUpdate {}

impl Default for TargetBinderUpdate {
    fn default() -> Self {
        TargetBinderUpdate {}
    }
}

#[derive(Clone, Indexed, ToAnyTrait)]
pub struct TargetBinderDelete {}

impl Default for TargetBinderDelete {
    fn default() -> Self {
        TargetBinderDelete {}
    }
}
