use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use colored::Colorize;
use pkg_select_shared::common_models::models::runtime_config::RuntimeConfig;
use pkg_select_shared::config_resolver::config_resolver::ConfigResolver;
use pkg_select_shared::{MutationLocker, safe_canonicalize};
use pkg_select_shared::argument_parser::models::argument::Argument;
use pkg_select_shared::common_models::models::configurations::target_binder::TargetBinder;
use pkg_select_shared::common_models::models::errors::canonical_path_combo::IOCanonicalError;
use pkg_select_shared::common_models::traits::binder_converter::BinderConverter;
use crate::models::commands::update_subcommands::target_binder::{TargetBinder as TargetBinderCommand, TargetBinderDelete, TargetBinderUpdate};
use crate::models::errors::command::CommandError;
use crate::models::trait_combos::command_combo::CommandOrCollection;
use crate::traits::cli_command::CLICommand;
use crate::traits::command_collection::CommandCollection;

impl CommandCollection for TargetBinderCommand {
    fn collection_names(&self) -> Vec<&str> {
        vec!["binder", "target-binder"]
    }

    fn commands(&self) -> Vec<CommandOrCollection> {
        vec![
            CommandOrCollection::Command(Rc::new(TargetBinderUpdate {})),
            CommandOrCollection::Command(Rc::new(TargetBinderDelete {})),
        ]
    }
}

impl CLICommand for TargetBinderUpdate {
    fn command_names(&self) -> Vec<&str> {
        vec!["update", "up"]
    }

    fn main(&self, _: &ConfigResolver, config: &mut MutationLocker<RuntimeConfig>, args: &Argument) -> Result<(), Vec<CommandError>> {
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

        let _category_name = args.optional_argument(format!("--category-name"))
            .or(args.optional_argument(format!("--category")));
        let category_name = match _category_name {
            None => {
                return Err(vec![
                    CommandError::String(
                        format!("category name is undefined.")
                    ),
                    CommandError::String(
                        format!("Hint: add --category-name=<name>")
                    )
                ])
            }
            Some(value) => value,
        };

        let _package_name = args.optional_argument(format!("--package-name"))
            .or(args.optional_argument(format!("--package")));
        let package_name = match _package_name {
            None => {
                return Err(vec![
                    CommandError::String(
                        format!("package name is undefined.")
                    ),
                    CommandError::String(
                        format!("Hint: add --package-name=<name>")
                    )
                ])
            }
            Some(value) => value,
        };

        let binders: HashMap<String, String> = args.non_optional.iter()
            .filter(| value | value.contains("="))
            .map(| value | {
                let split: Vec<&str> = value.split("=").collect();

                (split[0].to_string(), split[1].to_string())
            })
            .collect();

        return match binders.is_empty() {
            true => {
                Err(vec![
                    CommandError::String(
                        format!("no binders selected")
                    )
                ])
            }
            false => {
                update(&category_name, &package_name, &binders, config_mut)
            }
        }
    }
}

fn update(category_name: &String, package_name: &String, values: &HashMap<String, String>, config: &mut RuntimeConfig) -> Result<(), Vec<CommandError>> {
    let _category = config.package_categories.iter_mut()
        .find(| value | value.name.eq(category_name));
    let category = match _category {
        None => {
            return Err(vec![
                CommandError::String(
                    format!("category not selected")
                )
            ]);
        },
        Some(value) => value,
    };

    let _package = category.packages.iter_mut()
        .find(| value | value.name.eq(package_name));
    let package = match _package {
        None => {
            return Err(vec![
                CommandError::String(
                    format!("package not selected")
                )
            ]);
        },
        Some(value) => value,
    };

    for (target, exec) in values.iter() {
        let converted_target = match safe_canonicalize(&PathBuf::from(target)) {
            Ok(value) => value,
            Err(error) => {
                return Err(vec![
                   CommandError::Others(Box::new(error))
                ]);
            }
        };
        let _target_filename = converted_target.file_name()
            .map(| value | value.to_string_lossy().to_string());
        let target_filename = match _target_filename {
            None => {
                return Err(vec![
                    CommandError::String(
                        format!("{}", target.yellow())
                    ),
                    CommandError::String(
                        format!("{}", "target filename not selected".bright_red())
                    ),
                    CommandError::String(
                        format!("Hint: Make sure there is filename on the left path")
                    )
                ]);
            },
            Some(value) => value,
        };
        let _target_path = converted_target.parent()
            .map(| value | value.to_string_lossy().to_string());
        let target_path = match _target_path {
            None => {
                return Err(vec![
                    CommandError::String(
                        format!("{}", target.yellow())
                    ),
                    CommandError::String(
                        format!("{}", "Package Select cannot run with root path of drive".bright_red())
                    ),
                    CommandError::String(
                        format!("Hint: Append folder path on the left path")
                    ),
                ]);
            },
            Some(value) => value,
        };

        let exec_path = match safe_canonicalize(&PathBuf::from(exec)) {
            Ok(value) => value,
            Err(error) => {
                return Err(vec![
                    CommandError::Others(Box::new(error))
                ]);
            }
        };
        let exec_path_stringified = (&exec_path).to_string_lossy().to_string();

        let existed = package.binders.iter().position(| value | {
            value.target_name.eq(&target_filename) &&
                value.execution_path.eq(&exec_path_stringified)
        });

        match existed {
            None => {
                package.binders.push(TargetBinder {
                    target_name: (&target_filename).to_string(),
                    target_path,
                    execution_path: exec_path_stringified
                });

                println!("Added target binder {} runnable package {}.", &target_filename, package.name);
            }
            Some(_) => {
                return Err(vec![
                    CommandError::String(
                        format!("{}", exec_path.join(target_filename).to_string_lossy())
                    ),
                    CommandError::String(
                        format!("Execution path is duplicated")
                    )
                ]);
            }
        }
    }

    Ok(())
}

impl CLICommand for TargetBinderDelete {
    fn command_names(&self) -> Vec<&str> {
        vec!["delete", "del"]
    }

    fn main(&self, _: &ConfigResolver, config: &mut MutationLocker<RuntimeConfig>, args: &Argument) -> Result<(), Vec<CommandError>> {
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

        let _category_name = args.optional_argument(format!("--category-name"))
            .or(args.optional_argument(format!("--category")));
        let category_name = match _category_name {
            None => {
                return Err(vec![
                    CommandError::String(
                        format!("category name is undefined.")
                    ),
                    CommandError::String(
                        format!("Hint: add --category-name=<name>")
                    )
                ])
            }
            Some(value) => value,
        };

        let _package_name = args.optional_argument(format!("--package-name"))
            .or(args.optional_argument(format!("--package")));
        let package_name = match _package_name {
            None => {
                return Err(vec![
                    CommandError::String(
                        format!("package name is undefined.")
                    ),
                    CommandError::String(
                        format!("Hint: add --package-name=<name>")
                    )
                ])
            }
            Some(value) => value,
        };

        return match args.non_optional.is_empty() {
            true => {
                Err(vec![
                    CommandError::String(
                        format!("no binders selected")
                    )
                ])
            }
            false => {
                delete(&category_name, &package_name, &args.non_optional, config_mut)
            }
        }
    }
}

fn delete(category_name: &String, package_name: &String, names: &Vec<String>, config: &mut RuntimeConfig) -> Result<(), Vec<CommandError>> {
    let _category = config.package_categories.iter_mut()
        .find(| value | value.name.eq(category_name));
    let category = match _category {
        None => {
            return Err(vec![
                CommandError::String(
                    format!("category not selected")
                )
            ])
        },
        Some(value) => value,
    };

    let _package = category.packages.iter_mut()
        .find(| value | value.name.eq(package_name));
    let package = match _package {
        None => {
            return Err(vec![
                CommandError::String(
                    format!("package not selected")
                )
            ])
        },
        Some(value) => value,
    };

    let _converted: Result<Vec<String>, IOCanonicalError> = names.iter()
        .map(| value | {
            safe_canonicalize(&PathBuf::from(value))
                .map(| value | value.to_string_lossy().to_string())
        })
        .collect();
    let converted = match _converted {
        Ok(value) => value,
        Err(error) => {
            return Err(vec![
                CommandError::Others(Box::new(error))
            ])
        }
    };

    let mut execution_paths: HashMap<String, usize> = package.binders.iter()
        .enumerate()
        .map(| (index, value) | {
            (value.convert_exec_to_pathbuf().to_string_lossy().to_string(), index)
        })
        .collect();
    let deleted_path_indices: Vec<usize> = vec![];

    for value in (&converted).iter() {
        let existed = execution_paths.remove(value);

        match existed {
            None => {
                println!("Couldn't find target binder {}. Skipping...", value);
            }
            Some(_) => {
                println!("Removed target binder {}.", value);
            }
        }
    }
    for value in (&deleted_path_indices).iter() {
        package.binders.remove(*value);
    }

    Ok(())
}
