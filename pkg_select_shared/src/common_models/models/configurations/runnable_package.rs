use std::collections::HashMap;
use crate::common_models::models::configurations::target_binder::TargetBinder;
use crate::common_models::models::validatable::Validatable;
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunnablePackage {
    pub name: String,
    #[serde_as(as = "HashMap<DisplayFromStr, DisplayFromStr>")]
    pub envs: HashMap<String, String>,
    pub binders: Vec<TargetBinder>,
    pub included_paths: Vec<String>,
    pub excluded_paths: Vec<String>,
    #[serde(skip)]
    pub(in crate::common_models) validatable: Validatable,
}

impl Default for RunnablePackage {
    fn default() -> Self {
        RunnablePackage {
            name: "".to_string(),
            envs: HashMap::default(),
            binders: vec![],
            included_paths: vec![],
            excluded_paths: vec![],
            validatable: Default::default()
        }
    }
}
