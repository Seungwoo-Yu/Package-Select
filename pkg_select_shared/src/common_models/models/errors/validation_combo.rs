use crate::common_models::models::errors::canonical_path_combo::IOCanonicalError;
use crate::common_models::models::errors::validation::ValidationError;

#[derive(Debug)]
pub enum IOCanonicalSerdeValidationError {
    SerdeError(serde_json::Error),
    ValidationError(ValidationError),
    IOCanonicalError(IOCanonicalError),
}
