use std::rc::Rc;
use pkg_select_shared::common_models::models::configurations::package_category::PackageCategory;
use pkg_select_shared::common_models::models::runtime_config::RuntimeConfig;
use pkg_select_shared::config_resolver::config_resolver::ConfigResolver;
use pkg_select_shared::{MutationLocker, read_input};
use pkg_select_shared::argument_parser::models::argument::Argument;
use crate::models::commands::update_subcommands::package_category::{PackageCategory as PackageCategoryCommand, PackageCategoryDelete, PackageCategoryUpdate};
use crate::models::errors::command::CommandError;
use crate::models::trait_combos::command_combo::CommandOrCollection;
use crate::traits::cli_command::CLICommand;
use crate::traits::command_collection::CommandCollection;

impl CommandCollection for PackageCategoryCommand {
    fn collection_names(&self) -> Vec<&str> {
        vec!["category", "package-category"]
    }

    fn commands(&self) -> Vec<CommandOrCollection> {
        vec![
            CommandOrCollection::Command(Rc::new(PackageCategoryUpdate {})),
            CommandOrCollection::Command(Rc::new(PackageCategoryDelete {})),
        ]
    }
}

impl CLICommand for PackageCategoryUpdate {
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

        let targets: Vec<PackageCategory> = args.non_optional.iter()
            .map(| value | {
                let mut instance = PackageCategory::default();
                (&mut instance).name = value.to_string();

                instance
            })
            .collect();

        return match targets.is_empty() {
            true => {
                Err(vec![
                    CommandError::String(
                        format!("no categories selected")
                    )
                ])
            }
            false => {
                update(&targets, config_mut);

                Ok(())
            }
        };
    }
}

fn update(values: &Vec<PackageCategory>, config: &mut RuntimeConfig) -> () {
    let mut added_names: Vec<String> = values.iter()
        .map(| value | value.name.to_string())
        .collect();
    let mut duplicated_indices: Vec<usize> = config.package_categories.iter()
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

    for (index, value) in values.iter().enumerate() {
        let duplicated_index = duplicated_indices.iter()
            .position(| value | index.eq(value));

        match duplicated_index {
            None => {
                config.package_categories.push(value.clone());

                println!("Added package category {}.", value.name);
            }
            Some(value2) => {
                duplicated_indices.remove(value2);

                println!("Package category {} already exists. Would you like to replace it?", value.name);
                print!("> ");
                let input = read_input().expect("couldn't get input from terminal.").to_lowercase().replace("\n", "");
                if ((&input).len() == 1 && (&input).contains("y")) || (&input).eq(&format!("yes")) {
                    let default_package = config.package_categories[index].default_package;
                    let packages = config.package_categories[index].packages.clone();

                    config.package_categories[index] = value.clone();
                    config.package_categories[index].default_package = default_package;
                    config.package_categories[index].packages = packages;

                    println!("Updated package category {}.", value.name);
                } else {
                    println!("Skipped package category {}.", value.name);
                }
            }
        }
    }
}

impl CLICommand for PackageCategoryDelete {
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

        return match &args.non_optional.is_empty() {
            true => {
                Err(vec![
                    CommandError::String(
                        format!("no categories selected")
                    )
                ])
            }
            false => {
                delete(&args.non_optional, config_mut);

                Ok(())
            }
        }
    }
}

fn delete(names: &Vec<String>, config: &mut RuntimeConfig) -> () {
    for value in names.iter() {
        let target_index = config.package_categories.iter().position(| value2 | value2.name.eq(value));

        match target_index {
            None => {
                println!("couldn't find package category {}. Skipping...", value);
            }
            Some(value2) => {
                config.package_categories.remove(value2);

                println!("Removed package category {}.", value);
            }
        }
    }
}