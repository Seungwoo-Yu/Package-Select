use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TargetBinder {
    pub target_name: String,
    pub target_path: String,
    pub execution_path: String,
}
