use std::path::PathBuf;
use pkg_select_shared::common_models::models::configurations::runnable_package::RunnablePackage;
use pkg_select_shared::config_resolver::config_resolver::ConfigResolver;
use pkg_select_shared::config_resolver::traits::category_search::CategorySearch;
use pkg_select_shared::config_resolver::traits::config_persistence::ConfigPersistence;
use pkg_select_shared::config_resolver::traits::package_search::PackageSearch;
use pkg_select_shared::common_models::models::runtime_config::RuntimeConfig;
use pkg_select_shared::common_models::traits::validator::Validator;
use pkg_select_shared::{current_exec_file_path, MutationLocker, PathPop, print_dbg_on_debug, project_filename};
use pkg_select_shared::argument_parser::models::argument::Argument;
use pkg_select_shared::common_models::traits::binder_converter::BinderConverter;
use pkg_select_shared::common_models::traits::hashable_result::HashableResult;
use pkg_select_shared::ProjectType::ProjectSelectRunner;
use crate::impls::path_registration::{check_path_registered, path_registration_resolver};
use crate::models::binder_registration_resolver::BinderRegistrationResolver;
use crate::models::commands::validate::Validate;
use crate::models::errors::command::CommandError;
use crate::models::errors::path_binder_registration::PathBinderRegistrationError;
use crate::models::errors::path_binder_registration::Type::{BinderNotRegistered, PathNotRegistered};
use crate::models::errors::path_binder_registration_combo::DirectoryIOPathBinderRegistrationError;
use crate::models::path_registration_resolver::PathRegistrationResolver;
use crate::traits::binder_registration::BinderRegistration;
use crate::traits::cli_command::CLICommand;

impl CLICommand for Validate {
    fn command_names(&self) -> Vec<&str> {
        vec!["validate", "val"]
    }

    fn main(&self, resolver: &ConfigResolver, config: &mut MutationLocker<RuntimeConfig>, args: &Argument) -> Result<(), Vec<CommandError>> {
        let target_arg = args.optional_argument(format!("--target"));
        let skip_registration = args.optional_flag(format!("--skip-registration"));

        print_dbg_on_debug!(&target_arg);

        let process_file_path = match current_exec_file_path() {
            Ok(value) => value,
            Err(error) => {
                return Err(vec![CommandError::Others(Box::new(error))]);
            }
        };
        let process_path_without_filename = process_file_path.pop_path();
        let project_filename = project_filename(ProjectSelectRunner);

        let mut copied = config.value_mut().unwrap();

        let binder_registration_resolver = BinderRegistrationResolver {};
        let path_registration_resolver = match path_registration_resolver() {
            Ok(value) => value,
            Err(error) => {
                return Err(vec![CommandError::Others(Box::new(error))]);
            }
        };

        match &target_arg {
            None => {
                match copied.validate() {
                    Ok(_) => {}
                    Err(error) => {
                        return Err(vec![CommandError::Others(Box::new(error))]);
                    }
                };

                if !skip_registration {
                    for value in copied.package_categories.iter() {
                        for value2 in value.packages.iter() {
                            match validate_binders_in_package(
                                &process_path_without_filename,
                                &project_filename,
                                &binder_registration_resolver,
                                &path_registration_resolver,
                                value2,
                            ) {
                                Ok(_) => {}
                                Err(error) => {
                                    return Err(vec![CommandError::Others(Box::new(error))]);
                                }
                            }
                        }
                    }
                }
            },
            Some(value) => {
                let mut ever_found = false;

                let found_category = resolver.category_resolver.find_by_name(copied, value);
                match found_category {
                    None => {}
                    Some(value) => {
                        ever_found = true;

                        match value.clone().validate() {
                            Ok(_) => {}
                            Err(error) => {
                                return Err(vec![CommandError::Others(Box::new(error))]);
                            }
                        }

                        if !skip_registration {
                            for value in value.packages.iter() {
                                match validate_binders_in_package(
                                    &process_path_without_filename,
                                    &project_filename,
                                    &binder_registration_resolver,
                                    &path_registration_resolver,
                                    value,
                                ) {
                                    Ok(_) => {}
                                    Err(error) => {
                                        return Err(vec![CommandError::Others(Box::new(error))]);
                                    }
                                }
                            }
                        }
                    }
                }

                if !ever_found {
                    let found_package = resolver.package_resolver.find_by_name(copied, value);
                    match found_package {
                        None => {}
                        Some(value) => {
                            ever_found = true;

                            match value.clone().validate() {
                                Ok(_) => {},
                                Err(error) => {
                                    return Err(vec![CommandError::Others(Box::new(error))]);
                                }
                            }

                            if !skip_registration {
                                match validate_binders_in_package(
                                    &process_path_without_filename,
                                    &project_filename,
                                    &binder_registration_resolver,
                                    &path_registration_resolver,
                                    value,
                                ) {
                                    Ok(_) => {}
                                    Err(error) => {
                                        return Err(vec![CommandError::Others(Box::new(error))]);
                                    }
                                }
                            }
                        }
                    }

                    if !ever_found {
                        return Err(vec![
                            CommandError::String(
                                format!("not found")
                            )
                        ]);
                    }
                }
            }
        };

        match &target_arg {
            None => {
                match copied.hash() {
                    Ok(value) => {
                        if !copied.package_category_hash.eq(&value) {
                            copied.package_category_hash = value;

                            match resolver.update(&copied) {
                                Ok(_) => {}
                                Err(error) => {
                                    return Err(vec![
                                        CommandError::String(
                                            format!("couldn't update config for hash.")
                                        ),
                                        CommandError::Others(Box::new(error))
                                    ]);
                                }
                            };
                        }
                    }
                    Err(error) => {
                        return Err(vec![
                            CommandError::String(
                                format!("couldn't create hash for config.")
                            ),
                            CommandError::Others(Box::new(error))
                        ]);
                    }
                }
            }
            Some(_) => {}
        }

        println!("Validated successfully!");

        Ok(())
    }
}

fn validate_binders_in_package(
    process_path_without_filename: &PathBuf,
    source_name: &String,
    binder_resolver: &BinderRegistrationResolver,
    path_resolver: &PathRegistrationResolver,
    package: &RunnablePackage,
) -> Result<(), DirectoryIOPathBinderRegistrationError> {
    for value in package.binders.iter() {
        let binder_registered = match binder_resolver.registered(
            &value.convert_exec_to_pathbuf(),
            process_path_without_filename,
            source_name,
        ) {
            Ok(value) => value,
            Err(error) => {
                return Err(DirectoryIOPathBinderRegistrationError::from(error));
            }
        };

        if !binder_registered {
            return Err(
                DirectoryIOPathBinderRegistrationError::PathBinderRegistrationError(
                    PathBinderRegistrationError {
                        error_type: BinderNotRegistered(value.target_name.to_string())
                    }
                )
            );
        }

        let path_registered = match check_path_registered(
            path_resolver,
            &value
        ) {
            Ok(value) => value,
            Err(error) => {
                return Err(DirectoryIOPathBinderRegistrationError::from(error));
            }
        };

        if !path_registered {
            return Err(
                DirectoryIOPathBinderRegistrationError::PathBinderRegistrationError(
                    PathBinderRegistrationError {
                        error_type: PathNotRegistered(value.target_name.to_string())
                    }
                )
            );
        }
    }

    Ok(())
}
