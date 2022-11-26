use std::collections::HashMap;
use std::rc::Rc;
use pkg_select_shared::common_models::models::runtime_config::RuntimeConfig;
use pkg_select_shared::config_resolver::config_resolver::ConfigResolver;
use pkg_select_shared::MutationLocker;
use pkg_select_shared::argument_parser::models::argument::Argument;
use crate::models::commands::update_subcommands::env_var::{EnvVar, EnvVarDelete, EnvVarUpdate};
use crate::models::errors::command::CommandError;
use crate::models::trait_combos::command_combo::CommandOrCollection;
use crate::traits::cli_command::CLICommand;
use crate::traits::command_collection::CommandCollection;

impl CommandCollection for EnvVar {
    fn collection_names(&self) -> Vec<&str> {
        vec!["env", "env-var"]
    }

    fn commands(&self) -> Vec<CommandOrCollection> {
        vec![
            CommandOrCollection::Command(Rc::new(EnvVarUpdate {})),
            CommandOrCollection::Command(Rc::new(EnvVarDelete {})),
        ]
    }
}

impl CLICommand for EnvVarUpdate {
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

        let vars: HashMap<String, String> = args.non_optional.iter()
            .filter(| value | value.contains("="))
            .map(| value | {
                let split: Vec<&str> = value.split("=").collect();

                (split[0].to_string(), split[1].to_string())
            })
            .collect();

        return match vars.is_empty() {
            true => {
                Err(vec![
                    CommandError::String(
                        format!("no vars selected")
                    )
                ])
            }
            false => {
                update(&category_name, &package_name, &vars, config_mut)
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

    for (key, value) in values.iter() {
        let existed = package.envs.insert(key.to_string(), value.to_string());

        match existed {
            None => {
                println!("Added env var {} into runnable package {}.", key, package.name);
            }
            Some(_) => {
                println!("Updated env var {} into runnable package {}.", key, package.name);
            }
        }
    }

    Ok(())
}

impl CLICommand for EnvVarDelete {
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
                        format!("no vars selected")
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

    for value in names.iter() {
        let removed = package.envs.remove(value);

        match removed {
            None => {
                println!("couldn't find package category {}. Skipping...", value);
            }
            Some(_) => {
                println!("Removed package category {}.", value);
            }
        }
    }

    Ok(())
}
