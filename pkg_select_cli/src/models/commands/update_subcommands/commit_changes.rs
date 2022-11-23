use struct_indexer_macro::{Indexed, ToAnyTrait};

#[derive(Clone, Indexed, ToAnyTrait)]
pub struct CommitChanges {}

impl Default for CommitChanges {
    fn default() -> Self {
        CommitChanges {}
    }
}