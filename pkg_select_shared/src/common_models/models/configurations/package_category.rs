use crate::common_models::models::configurations::runnable_package::RunnablePackage;
use crate::common_models::models::validatable::Validatable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageCategory {
    pub name: String,
    pub packages: Vec<RunnablePackage>,
    pub default_package: Option<usize>,
    #[serde(skip)]
    pub(in crate::common_models) validatable: Validatable,
}

impl Default for PackageCategory {
    fn default() -> Self {
        PackageCategory {
            name: "".to_string(),
            packages: vec![],
            default_package: Some(0),
            validatable: Default::default(),
        }
    }
}
