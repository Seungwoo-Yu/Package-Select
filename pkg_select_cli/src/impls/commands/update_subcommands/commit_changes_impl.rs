use std::path::PathBuf;
use pkg_select_shared::argument_parser::models::argument::Argument;
use pkg_select_shared::common_models::models::runtime_config::RuntimeConfig;
use pkg_select_shared::common_models::traits::binder_converter::BinderConverter;
use pkg_select_shared::common_models::traits::hashable_result::HashableResult;
use pkg_select_shared::config_resolver::config_resolver::ConfigResolver;
use pkg_select_shared::config_resolver::traits::config_persistence::ConfigPersistence;
use pkg_select_shared::{current_exec_file_path, MutationLocker, PathPop, print_dbg_on_debug, project_filename};
use pkg_select_shared::ProjectType::ProjectSelectRunner;
use crate::impls::path_registration::{check_raw_path_registered, path_registration_resolver, register_raw_paths, unregister_raw_paths};
use crate::models::binder_registration_resolver::BinderRegistrationResolver;
use crate::models::commands::update_subcommands::commit_changes::CommitChanges;
use crate::models::commands::validate::Validate;
use crate::models::errors::command::CommandError;
use crate::traits::binder_registration::BinderRegistration;
use crate::traits::cli_command::CLICommand;

impl CLICommand for CommitChanges {
    fn command_names(&self) -> Vec<&str> {
        vec!["commit", "commit-changes"]
    }

    fn main(&self, resolver: &ConfigResolver, config: &mut MutationLocker<RuntimeConfig>, _: &Argument) -> Result<(), Vec<CommandError>> {
        let existed = match resolver.resolve() {
            Ok(value) => value,
            Err(error) => {
                return Err(vec![
                    CommandError::String(
                        format!("couldn't resolve config.")
                    ),
                    CommandError::Others(Box::new(error))
                ]);
            }
        };
        let existed_hash = match existed.hash() {
            Ok(value) => value,
            Err(error) => {
                return Err(vec![
                    CommandError::String(
                        format!("couldn't get hash for existed config.")
                    ),
                    CommandError::Others(Box::new(error))
                ]);
            }
        };
        let config_mut = match config.value_mut() {
            None => {
                return Err(vec![
                    CommandError::String(
                        format!("couldn't get mutable config")
                    )
                ])
            }
            Some(value) => value,
        };
        let changed_hash = match config_mut.hash() {
            Ok(value) => value,
            Err(error) => {
                return Err(vec![
                    CommandError::String(
                        format!("couldn't get hash for changed config.")
                    ),
                    CommandError::Others(Box::new(error))
                ]);
            }
        };

        if (&existed_hash).eq(&changed_hash) {
            return Err(vec![
                CommandError::String(
                    format!("nothing is changed. Aborting...")
                ),
            ]);
        }

        dbg!(&config_mut);

        if config_mut.package_categories.len() > 0 {
            let validator = Validate {};
            let mut validator_mutation_locker = MutationLocker::create(config_mut, true);
            let mut validator_args = Argument::default();
            validator_args.set_optional(format!("--skip-registration"), None);
            match validator.main(resolver, &mut validator_mutation_locker, &validator_args) {
                Ok(_) => false,
                Err(mut error) => {
                    error.insert(
                        0,
                        CommandError::String(
                            format!("failed validation before saving...")
                        )
                    );
                    return Err(error);
                }
            };
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
                return Err(vec![
                    CommandError::Others(Box::new(error))
                ]);
            }
        };

        let mut existed_info: Vec<PathBuf> = (&existed).package_categories.iter()
            .map(| value | {
                value.packages.iter()
                    .map(| value2 | {
                        value2.binders.iter()
                            .map(| value3 | value3.convert_exec_to_pathbuf())
                            .collect::<Vec<PathBuf>>()
                    })
                    .collect::<Vec<Vec<PathBuf>>>()
            })
            .flatten()
            .flatten()
            .collect();

        let updated_info: Vec<PathBuf> = (&config_mut).package_categories.iter()
            .map(| value | {
                value.packages.iter()
                    .map(| value2 | {
                        value2.binders.iter()
                            .map(| value3 | value3.convert_exec_to_pathbuf())
                            .collect::<Vec<PathBuf>>()
                    })
                    .collect::<Vec<Vec<PathBuf>>>()
            })
            .flatten()
            .flatten()
            .collect();

        let mut __sync_targets: Vec<&PathBuf> = updated_info.iter()
            .map(| value | {
                if existed_info.len() == 0 {
                    return Some(value);
                }

                let found_existed_index = match existed_info.iter()
                    .position(| value2 | value.eq(value2)) {
                    None => {
                        return Some(value);
                    },
                    Some(value) => value,
                };

                let found_path = existed_info.remove(found_existed_index);
                let binder_registered = match binder_registration_resolver.registered(
                    &found_path,
                    &process_path_without_filename,
                    &project_filename,
                ) {
                    Ok(value) => value,
                    Err(_) => false,
                };
                if !binder_registered {
                    return Some(value);
                }


                let path_registered = match check_raw_path_registered(&path_registration_resolver, value) {
                    Ok(value) => value,
                    Err(_) => false,
                };
                if !path_registered {
                    return Some(value);
                }

                None
            })
            .filter(| value | value.is_some())
            .map(| value | value.unwrap())
            .collect();

        let _sync_targets: Result<Vec<&PathBuf>, Vec<CommandError>> = __sync_targets.iter()
            .map(| value | {
                let binder_registered = match binder_registration_resolver.registered(
                    value,
                    &process_path_without_filename,
                    &project_filename,
                ) {
                    Ok(value2) => value2,
                    Err(error) => {
                        return Err(vec![
                            CommandError::Others(Box::new(error))
                        ]);
                    }
                };

                if !binder_registered {
                    match binder_registration_resolver.register(
                        value,
                        &process_path_without_filename,
                        &project_filename,
                    ) {
                        Ok(_) => {},
                        Err(error) => {
                            return Err(vec![
                                CommandError::Others(Box::new(error))
                            ]);
                        }
                    }
                }

                let path_registered = match check_raw_path_registered(&path_registration_resolver, value) {
                    Ok(value2) => value2,
                    Err(error) => {
                        return Err(vec![
                            CommandError::Others(Box::new(error))
                        ]);
                    }
                };

                if !path_registered {
                    return Ok(*value);
                }

                Ok(*value)
            })
            .collect();

        let sync_targets = match _sync_targets {
            Ok(value) => value,
            Err(error) => {
                return Err(error);
            }
        };

        match register_raw_paths(&mut path_registration_resolver, &sync_targets) {
            Ok(_) => {}
            Err(error) => {
                return Err(vec![
                    CommandError::Others(Box::new(error))
                ]);
            }
        }

        let desync_targets: Vec<PathBuf> = existed_info;

        let desync_target_refs: Vec<&PathBuf> = desync_targets.iter().collect();

        match unregister_raw_paths(
            &mut path_registration_resolver,
            &desync_target_refs
        ) {
            Ok(_) => {},
            Err(error) => {
                print_dbg_on_debug!(&error);
            }
        }

        for value in (&desync_targets).iter() {
            match binder_registration_resolver.unregister(
                value,
            ) {
                Ok(_) => {},
                Err(error) => {
                    print_dbg_on_debug!(&error);
                }
            }
        }

        config_mut.package_category_hash = changed_hash;

        dbg!(&config_mut);

        match resolver.update(config_mut) {
            Ok(_) => {}
            Err(error) => {
                return Err(vec![
                   CommandError::Others(Box::new(error))
                ]);
            }
        }

        println!("Saved changes successfully!");

        Ok(())
    }
}
