use std::io::ErrorKind;
use colored::Colorize;
use pkg_select_shared::argument_parser::models::argument::Argument;
use pkg_select_shared::common_models::models::runtime_config::RuntimeConfig;
use pkg_select_shared::config_resolver::config_resolver::ConfigResolver;
use pkg_select_shared::{current_exec_file_path, MutationLocker, PathPop, project_filename, read_input};
use pkg_select_shared::common_models::traits::binder_converter::BinderConverter;
use pkg_select_shared::config_resolver::traits::config_persistence::ConfigPersistence;
use pkg_select_shared::ProjectType::ProjectSelectRunner;
use crate::impls::path_registration::{path_registration_resolver, reset_paths};
use crate::models::binder_registration_resolver::BinderRegistrationResolver;
use crate::models::commands::purge::Purge;
use crate::models::errors::binder_registration_combo::IOBinderRegistrationError;
use crate::models::errors::command::CommandError;
use crate::traits::binder_registration::BinderRegistration;
use crate::traits::cli_command::CLICommand;

impl CLICommand for Purge {
    fn command_names(&self) -> Vec<&str> {
        vec!["purge"]
    }

    fn main(&self, resolver: &ConfigResolver, config: &mut MutationLocker<RuntimeConfig>, args: &Argument) -> Result<(), Vec<CommandError>> {
        let skip_confirm = args.optional_flag(format!("--skip-confirm"));

        if !skip_confirm {
            println!("{}", "Every package config data and registration is about to be erased. It can result in occurring unexpected errors.".bright_red());
            println!("Type \"Confirm\" to continue.");

            let input = read_input().expect("couldn't get input from terminal.");
            if !input.to_lowercase().eq("confirm") {
                println!("Purge aborted.");
                return Ok(());
            }
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

        for value in config.value().package_categories.iter() {
            for value2 in value.packages.iter() {
                for value3 in value2.binders.iter() {
                    let registered = match (&binder_registration_resolver).registered(
                        &value3.convert_exec_to_pathbuf(),
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
                            &value3.convert_exec_to_pathbuf()
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

        match reset_paths(
            &mut path_registration_resolver
        ) {
            Ok(_) => {},
            Err(error) => {
                return Err(vec![CommandError::Others(Box::new(error))]);
            }
        };

        match resolver.reset() {
            Ok(_) => {}
            Err(error) => {
                return Err(vec![CommandError::Others(Box::new(error))]);
            }
        }

        println!("Purge completed.");

        Ok(())
    }
}
