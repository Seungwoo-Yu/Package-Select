use std::io::ErrorKind;
use colored::Colorize;
use pkg_select_shared::config_resolver::config_resolver::ConfigResolver;
use pkg_select_shared::common_models::models::configurations::target_binder::TargetBinder;
use pkg_select_shared::common_models::models::runtime_config::RuntimeConfig;
use pkg_select_shared::{current_exec_file_path, MutationLocker, PathPop, print_dbg_on_debug, project_filename};
use pkg_select_shared::argument_parser::models::argument::Argument;
use pkg_select_shared::common_models::traits::binder_converter::BinderConverter;
use pkg_select_shared::ProjectType::ProjectSelectRunner;
use crate::impls::path_registration::{filter_paths_registered, path_registration_resolver, unregister_paths};
use crate::models::binder_registration_resolver::BinderRegistrationResolver;
use crate::models::commands::desync::Desync;
use crate::models::errors::binder_registration_combo::IOBinderRegistrationError;
use crate::models::errors::command::CommandError;
use crate::traits::binder_registration::BinderRegistration;
use crate::traits::cli_command::CLICommand;

impl CLICommand for Desync {
    fn command_names(&self) -> Vec<&str> {
        vec!["desync"]
    }

    fn main(&self, _: &ConfigResolver, config: &mut MutationLocker<RuntimeConfig>, args: &Argument) -> Result<(), Vec<CommandError>> {
        let target_arg = args.optional_argument(format!("--target"));

        if (&target_arg).is_none() {
            println!("{}", "warning: target is not specified. It'll end up desyncing all packages.".yellow());
            println!("add --target={{name}} and run desync again to take effect.");
        }

        let process_file_path = match current_exec_file_path() {
            Ok(value) => value,
            Err(error) => {
                return Err(vec![CommandError::Others(Box::new(error))]);
            }
        };
        let process_path_without_filename = process_file_path.pop_path();
        let project_filename = project_filename(ProjectSelectRunner);

        let binder_registration_resolver = BinderRegistrationResolver {};
        let mut path_registration_resolver = match path_registration_resolver() {
            Ok(value) => value,
            Err(error) => {
                return Err(vec![CommandError::Others(Box::new(error))]);
            }
        };

        print_dbg_on_debug!(&target_arg);

        match &target_arg {
            None => {
                let mut binders: Vec<TargetBinder> = vec![];

                for value in config.value().package_categories.iter() {
                    for value2 in value.packages.iter() {
                        for value3 in value2.binders.iter() {
                            (&mut binders).push(value3.clone());
                        }
                    }
                }

                let path_registered_binders = match filter_paths_registered(
                    &path_registration_resolver,
                    &binders,
                ) {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(vec![CommandError::Others(Box::new(error))]);
                    }
                };

                match unregister_paths(&mut path_registration_resolver, &path_registered_binders) {
                    Ok(_) => {}
                    Err(error) => {
                        return Err(vec![CommandError::Others(Box::new(error))]);
                    }
                }

                for value in binders.iter() {
                    let registered = match (&binder_registration_resolver).registered(
                        &value.convert_exec_to_pathbuf(),
                        &process_path_without_filename,
                        &project_filename
                    ) {
                        Ok(value) => value,
                        Err(error) => {
                            if let IOBinderRegistrationError::IOError(value) = &error {
                                if value.kind() != ErrorKind::NotFound {
                                    return Err(vec![CommandError::Others(Box::new(error))]);
                                }
                            }

                            false
                        }
                    };

                    if registered {
                        match (&binder_registration_resolver).unregister(
                            &value.convert_exec_to_pathbuf(),
                        ) {
                            Ok(_) => {}
                            Err(error) => {
                                return Err(vec![CommandError::Others(Box::new(error))]);
                            }
                        }
                    }
                }
            }
            Some(value) => {
                let mut binders: Vec<TargetBinder> = vec![];

                let _category = config.value().package_categories.iter()
                    .find(| value2 | value2.name.eq(&value.to_string()));
                let category = match _category {
                    None => {
                        return Err(vec![
                            CommandError::String(
                                format!("No found package named {}.", value)
                            )
                        ]);
                    }
                    Some(value) => value
                };

                for value in category.packages.iter() {
                    for value2 in value.binders.iter() {
                        (&mut binders).push(value2.clone());
                    }
                }

                let path_registered_binders = match filter_paths_registered(
                    &path_registration_resolver,
                    &binders,
                ) {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(vec![CommandError::Others(Box::new(error))]);
                    }
                };

                match unregister_paths(&mut path_registration_resolver, &path_registered_binders) {
                    Ok(_) => {}
                    Err(error) => {
                        return Err(vec![CommandError::Others(Box::new(error))]);
                    }
                }

                for value in binders.iter() {
                    let registered = match (&binder_registration_resolver).registered(
                        &value.convert_exec_to_pathbuf(),
                        &process_path_without_filename,
                        &project_filename
                    ) {
                        Ok(value) => value,
                        Err(error) => {
                            if let IOBinderRegistrationError::IOError(value) = &error {
                                if value.kind() != ErrorKind::NotFound {
                                    return Err(vec![CommandError::Others(Box::new(error))]);
                                }
                            }

                            false
                        }
                    };

                    if registered {
                        match (&binder_registration_resolver).unregister(
                            &value.convert_exec_to_pathbuf(),
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

        println!("Desynced successfully!");

        Ok(())
    }
}
