use crate::common_models::models::configurations::package_category::PackageCategory;
use crate::common_models::models::errors::validation::Type::{
    EmptyPackageList, InvalidDefaultPackage,
};
use crate::common_models::models::errors::validation::ValidationError;
use crate::common_models::models::errors::validation_combo::IOCanonicalSerdeValidationError;
use crate::common_models::traits::validator::Validator;
use std::fmt::{Display, Formatter};

impl PartialEq for PackageCategory {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name) &&
            self.packages.eq(&other.packages) &&
            self.default_package.eq(&other.default_package)
    }
}

impl Validator for PackageCategory {
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

fn validate_internal(category: &mut PackageCategory) -> Result<(), IOCanonicalSerdeValidationError> {
    if category.packages.is_empty() {
        return Err(IOCanonicalSerdeValidationError::ValidationError(ValidationError {
            error_type: EmptyPackageList,
        }));
    }

    for value in category.packages.iter_mut() {
        if value.validated() {
            continue;
        }

        match value.validate() {
            Ok(_) => {}
            Err(error) => {
                return Err(error);
            }
        }
    }

    let default_package_index = match category.default_package {
        None => {
            return Err(IOCanonicalSerdeValidationError::ValidationError(ValidationError {
                error_type: InvalidDefaultPackage,
            }));
        }
        Some(value) => value,
    };

    if default_package_index >= category.packages.len() {
        return Err(IOCanonicalSerdeValidationError::ValidationError(ValidationError {
            error_type: InvalidDefaultPackage,
        }));
    }

    match category.packages.get(default_package_index) {
        None => {
            return Err(IOCanonicalSerdeValidationError::ValidationError(ValidationError {
                error_type: InvalidDefaultPackage,
            }));
        }
        Some(_) => {}
    }

    category.validatable.validated = true;
    Ok(())
}

fn invalidate(category: &mut PackageCategory) {
    for value in category.packages.iter_mut() {
        value.invalidate();
    }

    category.validatable.validated = false
}

impl Display for PackageCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
