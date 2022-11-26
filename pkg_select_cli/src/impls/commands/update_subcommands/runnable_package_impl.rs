use std::rc::Rc;
use pkg_select_shared::common_models::models::runtime_config::RuntimeConfig;
use pkg_select_shared::config_resolver::config_resolver::ConfigResolver;
use pkg_select_shared::{MutationLocker};
use pkg_select_shared::argument_parser::models::argument::Argument;
use pkg_select_shared::common_models::models::configurations::runnable_package::RunnablePackage;
use crate::models::commands::update_subcommands::runnable_package::{RunnablePackage as RunnablePackageCommand, RunnablePackageDelete, RunnablePackageUpdate};
use crate::models::errors::command::CommandError;
use crate::models::trait_combos::command_combo::CommandOrCollection;
use crate::traits::cli_command::CLICommand;
use crate::traits::command_collection::CommandCollection;

impl CommandCollection for RunnablePackageCommand {
    fn collection_names(&self) -> Vec<&str> {
        vec!["package", "runnable-package"]
    }

    fn commands(&self) -> Vec<CommandOrCollection> {
        vec![
            CommandOrCollection::Command(Rc::new(RunnablePackageUpdate {})),
            CommandOrCollection::Command(Rc::new(RunnablePackageDelete {})),
        ]
    }
}

impl CLICommand for RunnablePackageUpdate {
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

        dbg!(args);

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

        let targets: Vec<RunnablePackage> = args.non_optional.iter()
            .map(| value | {
                let mut instance = RunnablePackage::default();
                (&mut instance).name = value.to_string();

                instance
            })
            .collect();

        return match targets.is_empty() {
            true => {
                Err(vec![
                    CommandError::String(
                        format!("no packages selected")
                    )
                ])
            }
            false => {
                update(&category_name, &targets, config_mut)
            }
        };
    }
}

fn update(category_name: &String, values: &Vec<RunnablePackage>, config: &mut RuntimeConfig) -> Result<(), Vec<CommandError>> {
    let mut added_names: Vec<String> = values.iter()
        .map(| value | value.name.to_string())
        .collect();
    let duplicated_indices: Vec<usize> = config.package_categories.iter()
        .enumerate()
        .map(| (index, value) | {
            let duplicated_position = added_names.iter()
                .position(| value2 | value2.eq(&value.name));

            match duplicated_position {
                None => {}
                Some(value) => {
                    added_names.remove(value);
                    return Some(index);
                }
            }

            None
        })
        .filter(| value | value.is_some())
        .map(| value| value.unwrap())
        .collect();

    let _category = config.package_categories.iter_mut()
        .find(| value | value.name.eq(category_name));
    let category = match _category {
        None => {
            return Err(vec![
                CommandError::String(
                    format!("category not found")
                )
            ])
        },
        Some(value) => value,
    };

    for (index, value) in values.iter().enumerate() {
        let duplicated_index = duplicated_indices.iter()
            .position(| value | index.eq(value));

        match duplicated_index {
            None => {
                (&mut category.packages).push(value.clone());

                println!("Added runnable package {}.", value.name);
            }
            Some(_) => {
                return Err(vec![
                    CommandError::String(
                        format!("Name {} is duplicated", value.name)
                    )
                ]);
            }
        }
    }

    Ok(())
}

impl CLICommand for RunnablePackageDelete {
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

        return match &args.non_optional.is_empty() {
            true => {
                Err(vec![
                    CommandError::String(
                        format!("no packages selected")
                    )
                ])
            }
            false => {
                delete(&category_name, &args.non_optional, config_mut)
            }
        }
    }
}

fn delete(category_name: &String,  names: &Vec<String>, config: &mut RuntimeConfig) -> Result<(), Vec<CommandError>> {
    let _category = config.package_categories.iter_mut()
        .find(| value | value.name.eq(category_name));
    let mut category = match _category {
        None => {
            return Err(vec![
                CommandError::String(
                    format!("category not found")
                )
            ])
        },
        Some(value) => value,
    };

    for value in names.iter() {
        let target_index = category.packages.iter().position(| value2 | value2.name.eq(value));

        match target_index {
            None => {
                println!("couldn't find runnable package {}. Skipping...", value);
            }
            Some(value2) => {
                category.packages.remove(value2);

                match category.default_package {
                    None => {}
                    Some(value3) => {
                        if value2 == value3 {
                            category.default_package = None;
                        }
                    }
                }

                println!("Removed runnable package {}.", value);
            }
        }
    }

    Ok(())
}
