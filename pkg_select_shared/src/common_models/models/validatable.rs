use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Validatable {
    #[serde(skip)]
    pub(in crate::common_models) validated: bool,
}

impl Default for Validatable {
    fn default() -> Self {
        Validatable { validated: false }
    }
}
