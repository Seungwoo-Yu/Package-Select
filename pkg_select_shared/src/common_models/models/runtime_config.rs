use crate::common_models::models::configurations::package_category::PackageCategory;
use crate::common_models::models::validatable::Validatable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub package_category_hash: String,
    pub package_categories: Vec<PackageCategory>,
    #[serde(skip)]
    pub(crate) validatable: Validatable,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        RuntimeConfig {
            package_category_hash: "".to_string(),
            package_categories: vec![],
            validatable: Default::default(),
        }
    }
}
