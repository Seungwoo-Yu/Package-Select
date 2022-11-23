use crate::common_models::models::errors::validation::Type::{EmptyCategoryList, NonUniqueName, NonUniqueTargetPath};
use crate::common_models::models::errors::validation::ValidationError;
use crate::common_models::models::errors::validation_combo::IOCanonicalSerdeValidationError;
use crate::common_models::models::runtime_config::RuntimeConfig;
use crate::common_models::traits::hashable_result::HashableResult;
use crate::common_models::traits::validator::Validator;
use crate::{safe_canonicalize, string_to_hash};
use serde_json::Error;
use crate::common_models::models::errors::canonical_path_combo::IOCanonicalError;
use crate::common_models::traits::binder_converter::BinderConverter;

impl Validator for RuntimeConfig {
    fn validated(&self) -> bool {
        self.validatable.validated
    }

    fn validate(&mut self) -> Result<(), IOCanonicalSerdeValidationError> {
        validate_internal(self)
    }

    fn invalidate(&mut self) {
        invalidate(self);
    }
}

fn validate_internal(config: &mut RuntimeConfig) -> Result<(), IOCanonicalSerdeValidationError> {
    if config.package_categories.is_empty() {
        return Err(IOCanonicalSerdeValidationError::ValidationError(ValidationError {
            error_type: EmptyCategoryList,
        }));
    }

    let config_hash = match config.hash() {
        Ok(value) => value,
        Err(error) => {
            return Err(IOCanonicalSerdeValidationError::SerdeError(error));
        }
    };

    if config.package_category_hash.eq(&config_hash) {
        skip_validating_children(config);
        return Ok(());
    }

    for value in config.package_categories.iter_mut() {
        match value.validate() {
            Ok(_) => {},
            Err(error) => return Err(error),
        }
    }

    match validate_name_uniqueness(config) {
        None => {},
        Some(value) => {
            return Err(IOCanonicalSerdeValidationError::ValidationError(ValidationError {
                error_type: NonUniqueName(value.to_string()),
            }));
        }
    }

    match validate_target_path_uniqueness(config) {
        Ok(value) => {
            match value {
                None => {},
                Some(value) => {
                    return Err(IOCanonicalSerdeValidationError::ValidationError(ValidationError {
                        error_type: NonUniqueTargetPath(value.to_string()),
                    }));
                }
            }
        }
        Err(error) => {
            return Err(IOCanonicalSerdeValidationError::IOCanonicalError(error));
        }
    }

    config.validatable.validated = true;
    Ok(())
}

pub fn validate_name_uniqueness<'t>(config: &'t RuntimeConfig) -> Option<&String> {
    let mut names: Vec<&String> = vec![];

    for value in config.package_categories.iter() {
        if names.contains(&&value.name) {
            return Some(&value.name);
        }
        names.push(&value.name);

        for value2 in value.packages.iter() {
            if names.contains(&&value2.name) {
                return Some(&value2.name);
            }
            names.push(&value2.name);
        }
    }

    None
}

pub fn validate_target_path_uniqueness(config: &RuntimeConfig) -> Result<Option<String>, IOCanonicalError> {
    let mut paths: Vec<String> = vec![];

    for value in config.package_categories.iter() {
        for value2 in value.packages.iter() {
            for value3 in value2.binders.iter() {
                let current_path = safe_canonicalize(&value3.convert_target_to_pathbuf())?
                    .to_string_lossy()
                    .to_string();

                if paths.contains(&current_path) {
                    return Ok(Some(current_path));
                }

                paths.push(current_path);
            }
        }
    }

    Ok(None)
}

fn skip_validating_children(package: &mut RuntimeConfig) {
    for value in package.package_categories.iter_mut() {
        value.validatable.validated = true;

        for value2 in value.packages.iter_mut() {
            value2.validatable.validated = true;
        }
    }
}

fn invalidate(config: &mut RuntimeConfig) {
    for value in config.package_categories.iter_mut() {
        value.invalidate();
    }

    config.validatable.validated = false
}

impl HashableResult<Error> for RuntimeConfig {
    fn hash(&self) -> Result<String, Error> {
        let config_json = match serde_json::to_string(&self.package_categories) {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };

        Ok(string_to_hash(&config_json))
    }
}
