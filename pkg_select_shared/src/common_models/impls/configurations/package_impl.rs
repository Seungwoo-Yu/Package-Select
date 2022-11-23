use std::collections::HashSet;
use std::path::{Path, PathBuf};
use crate::common_models::models::configurations::runnable_package::RunnablePackage;
use crate::common_models::models::configurations::target_binder::TargetBinder;
use crate::common_models::models::errors::validation::Type::{DuplicatedBinderExecutionPath, DuplicatedExcludedPath, DuplicatedIncludedPath, DuplicatedPathInIncludedAndExcluded, EmptyBinderList, InvalidBinderPath, InvalidExcludedPath, InvalidIncludedPath};
use crate::common_models::models::errors::validation::ValidationError;
use crate::common_models::models::errors::validation_combo::IOCanonicalSerdeValidationError;
use crate::common_models::traits::binder_search::BinderSearch;
use crate::common_models::traits::validator::Validator;
use crate::{PathPop, safe_canonicalize};
use crate::common_models::models::errors::canonical_path_combo::IOCanonicalError;
use crate::common_models::traits::binder_converter::BinderConverter;

impl PartialEq for RunnablePackage {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name) &&
            self.binders.eq(&other.binders) &&
            self.excluded_paths.eq(&other.excluded_paths) &&
            self.included_paths.eq(&other.included_paths) &&
            self.envs.eq(&other.envs)
    }
}

impl Validator for RunnablePackage {
    fn validated(&self) -> bool {
        self.validatable.validated
    }

    fn validate(&mut self) -> Result<(), IOCanonicalSerdeValidationError> {
        validate_internal(self)
    }

    fn invalidate(&mut self) {
        self.validatable.validated = false
    }
}

fn validate_internal(package: &mut RunnablePackage) -> Result<(), IOCanonicalSerdeValidationError> {
    if package.binders.is_empty() {
        return Err(IOCanonicalSerdeValidationError::ValidationError(ValidationError {
            error_type: EmptyBinderList,
        }));
    }

    for value in package.binders.iter() {
        if !validate_binder_path(value) {
            package.validatable.validated = false;
            return Err(IOCanonicalSerdeValidationError::ValidationError(ValidationError {
                error_type: InvalidBinderPath,
            }));
        }
    }
    let binder_exec_path_set: Vec<String> = package.binders.iter()
        .map(| value | value.convert_exec_to_pathbuf().to_string_lossy().to_string())
        .collect();
    let binder_validated = match validate_paths_unique(&binder_exec_path_set) {
        Ok(value) => value,
        Err(error) => {
            return Err(IOCanonicalSerdeValidationError::IOCanonicalError(error));
        }
    };
    if !binder_validated {
        package.validatable.validated = false;
        return Err(IOCanonicalSerdeValidationError::ValidationError(ValidationError {
            error_type: DuplicatedBinderExecutionPath,
        }));
    }

    for value in package.included_paths.iter() {
        if !validate_path(value) {
            package.validatable.validated = false;
            return Err(IOCanonicalSerdeValidationError::ValidationError(ValidationError {
                error_type: InvalidIncludedPath,
            }));
        }
    }
    let included_paths_validated = match validate_paths_unique(&package.included_paths) {
        Ok(value) => value,
        Err(error) => {
            return Err(IOCanonicalSerdeValidationError::IOCanonicalError(error));
        }
    };
    if !included_paths_validated {
        package.validatable.validated = false;
        return Err(IOCanonicalSerdeValidationError::ValidationError(ValidationError {
            error_type: DuplicatedIncludedPath,
        }));
    }

    for value in package.excluded_paths.iter() {
        if !validate_path(value) {
            package.validatable.validated = false;
            return Err(IOCanonicalSerdeValidationError::ValidationError(ValidationError {
                error_type: InvalidExcludedPath,
            }));
        }
    }
    let excluded_paths_validated = match validate_paths_unique(&package.excluded_paths) {
        Ok(value) => value,
        Err(error) => {
            return Err(IOCanonicalSerdeValidationError::IOCanonicalError(error));
        }
    };
    if !excluded_paths_validated {
        package.validatable.validated = false;
        return Err(IOCanonicalSerdeValidationError::ValidationError(ValidationError {
            error_type: DuplicatedExcludedPath,
        }));
    }

    let working_paths: Vec<String> = (&package.included_paths).iter()
        .chain((&package.excluded_paths).iter())
        .map(| value | value.to_string())
        .collect();
    let working_paths_validated = match validate_paths_unique(&working_paths) {
        Ok(value) => value,
        Err(error) => {
            return Err(IOCanonicalSerdeValidationError::IOCanonicalError(error));
        }
    };
    if !working_paths_validated {
        package.validatable.validated = false;
        return Err(IOCanonicalSerdeValidationError::ValidationError(ValidationError {
            error_type: DuplicatedPathInIncludedAndExcluded,
        }));
    }

    package.validatable.validated = true;
    Ok(())
}

fn validate_binder_path(binder: &TargetBinder) -> bool {
    if binder.target_path.is_empty()
        || binder.target_name.is_empty()
        || binder.execution_path.is_empty()
    {
        return false;
    }

    true
}

fn validate_path(path: &String) -> bool {
    Path::new(path).exists()
}

fn validate_paths_unique(list: &Vec<String>) -> Result<bool, IOCanonicalError> {
    let paths: Result<HashSet<String>, IOCanonicalError> = list.iter()
        .map(| value | {
            safe_canonicalize(&PathBuf::from(value))
                .map(| value | value.to_string_lossy().to_string())
        })
        .collect();

    Ok(list.len() == paths?.len())
}

impl BinderSearch for RunnablePackage {
    fn find_binder_by_path<'t>(&'t self, path: &'t PathBuf) -> Option<&'t TargetBinder> {
        let safe_path = safe_canonicalize(path).ok()?;
        let filename = match (&safe_path).is_file() {
            true => (&safe_path).file_name()?,
            false => {
                return None;
            }
        };
        let path_without_filename = (&safe_path).pop_path();

        self.binders.iter().find(|value| {
            let execution_path = Path::new(&value.execution_path);

            filename.to_string_lossy().eq(&value.target_name) &&
                (&path_without_filename).eq(execution_path)
        })
    }
}
