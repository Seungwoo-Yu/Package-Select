use crate::common_models::models::errors::validation_combo::IOCanonicalSerdeValidationError;

pub trait Validator {
    fn validated(&self) -> bool;
    fn validate(&mut self) -> Result<(), IOCanonicalSerdeValidationError>;
    fn invalidate(&mut self);
}
